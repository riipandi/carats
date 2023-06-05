// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{net::SocketAddr, time::Duration};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Used for tracing layer (logger middleware)
use axum::{extract::MatchedPath, http::Request, response::Response, Router};
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};

use crate::utils;

pub static BASE_PATH_API: &str = "/api";
pub static BASE_PATH_SPA: &str = "/ui";

pub async fn run(addr: String) {
	// Define log tracing level
	let log_level_app = format!("{}=debug", crate::PKG_NAME);
	let log_level_lib = String::from("tower_http=info,axum::rejection=trace,sqlx=error");
	let filters = format!("{},{}", log_level_app, log_level_lib);

	// Start log tracing since server started
	tracing_subscriber::registry()
		.with(EnvFilter::try_from_default_env().unwrap_or_else(|_| filters.into()))
		.with(tracing_subscriber::fmt::layer())
		.init();

	// We can combine multiple services with different port.
	tokio::join!(app_service(addr));
}

async fn app_service(addr: String) {
	// Open database connection
	let conn_str = utils::get_envar("DATABASE_URL", None);
	let db_pool = sqlx::postgres::PgPoolOptions::new()
		.max_connections(5)
		.connect(&conn_str)
		.await
		.expect("Can't connect to database");

	let app = Router::new()
		.with_state(db_pool)
		.merge(crate::routes::root::register())
		.nest(BASE_PATH_API, crate::routes::api::register())
		.nest(BASE_PATH_SPA, crate::routes::spa::register());

	tracing::info!("🚀 Application started at http://{}", addr);
	serve(app, addr).await;
}

async fn serve(app: Router, addr: String) {
	let socket_addr: SocketAddr = addr.parse().expect("Unable to parse socket address");
	let logger = TraceLayer::new_for_http()
		.make_span_with(|req: &Request<_>| {
			// Log the matched route's path (with placeholders not filled in).
			// Use req.uri() or OriginalUri if you want the real path.
			let matched_path = req
				.extensions()
				.get::<MatchedPath>()
				.map(MatchedPath::as_str);

			info_span!(
				"http_request",
				method = ?req.method(),
				matched_path,
				some_other_field = tracing::field::Empty,
			)
		})
		.on_request(|req: &Request<_>, _span: &Span| {
			tracing::info!("Request {} {}", req.method(), req.uri());
		})
		.on_response(|res: &Response, latency: Duration, _span: &Span| {
			tracing::info!("Response {} {:?}", res.status(), latency);
		})
		.on_failure(
			|error: ServerErrorsFailureClass, latency: Duration, _span: &Span| {
				tracing::error!("Failure {} {:?}", error, latency);
			},
		);

	axum::Server::bind(&socket_addr)
		.serve(app.layer(logger).into_make_service())
		.with_graceful_shutdown(shutdown_signal())
		.await
		.unwrap();
}

// Graceful shutdown
async fn shutdown_signal() {
	let ctrl_c = async {
		tokio::signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		_ = ctrl_c => {},
		_ = terminate => {},
	}

	tracing::info!("signal received, starting graceful shutdown");
}

pub mod middleware {
	use axum::http::{HeaderValue, Method};
	use tower_http::cors::CorsLayer;

	pub fn cors() -> CorsLayer {
		CorsLayer::new()
			.allow_origin("*".parse::<HeaderValue>().unwrap())
			.allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
	}
}

pub mod responder {
	use axum::extract::{FromRef, FromRequestParts};
	use axum::http::request::Parts;
	use axum::http::{header, HeaderMap, StatusCode};
	use axum::response::{IntoResponse, Response};
	use axum::Json;

	use async_trait::async_trait;
	use serde::Serialize;
	use sqlx::{pool::PoolConnection, PgPool, Postgres};

	// Make our own error that wraps `anyhow::Error`.
	pub struct AppError(anyhow::Error);

	#[derive(Serialize, Debug)]
	pub struct ResponseError {
		pub status_code: u16,
		pub message: String,
		pub reason: Option<String>,
	}

	#[derive(Serialize, Debug)]
	pub struct JsonResponse<T> {
		pub status_code: u16,
		pub message: Option<&'static str>,
		pub data: Option<T>,
	}

	// Tell axum how to convert `AppError` into a response.
	impl IntoResponse for AppError {
		fn into_response(self) -> Response {
			let http_status = StatusCode::INTERNAL_SERVER_ERROR;

			let err = ResponseError {
				status_code: http_status.as_u16(),
				message: http_status.to_string(),
				reason: Some(format!("{}", self.0)),
			};

			(http_status, Json(err)).into_response()
		}
	}

	// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
	// `Result<_, AppError>`. That way you don't need to do that manually.
	impl<E> From<E> for AppError
	where
		E: Into<anyhow::Error>,
	{
		fn from(err: E) -> Self {
			Self(err.into())
		}
	}

	// we can also write a custom extractor that grabs a connection from the pool
	// which setup is appropriate depends on your application
	pub struct DatabaseConnection(PoolConnection<Postgres>);

	#[async_trait]
	impl<S> FromRequestParts<S> for DatabaseConnection
	where
		PgPool: FromRef<S>,
		S: Send + Sync,
	{
		type Rejection = (StatusCode, String);
		async fn from_request_parts(
			_parts: &mut Parts,
			state: &S,
		) -> Result<Self, Self::Rejection> {
			let pool = PgPool::from_ref(state);
			let conn = pool.acquire().await.map_err(internal_error)?;
			Ok(Self(conn))
		}
	}

	/// Utility function for mapping any error into a `500 Internal Server Error` response.
	pub fn internal_error<E>(err: E) -> (StatusCode, String)
	where
		E: std::error::Error,
	{
		(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
	}

	pub async fn throw_not_found() -> impl IntoResponse {
		let http_status = StatusCode::NOT_FOUND;
		(
			http_status,
			Json(ResponseError {
				status_code: http_status.as_u16(),
				message: String::from("Resource not found"),
				reason: None,
			}),
		)
	}

	pub fn as_plain(status_code: StatusCode, body: String) -> Response {
		let mut headers = HeaderMap::new();
		headers.insert(header::CONTENT_TYPE, "text/plain".parse().unwrap());
		(status_code, headers, body).into_response()
	}
}
