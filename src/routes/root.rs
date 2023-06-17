// Copyright 2023 Aris Ripandi, and Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use axum::response::{IntoResponse, Redirect};
use axum::{http::StatusCode, routing::*, Extension, Router};
use sqlx::PgPool;

use crate::server::BASE_PATH_API;
use crate::{routes, server::responder};

pub fn register() -> Router {
	Router::new()
		.merge(home_route())
		.merge(health_check())
		.fallback(responder::throw_not_found)
}

fn home_route() -> Router {
	async fn handler() -> impl IntoResponse {
		Redirect::to(BASE_PATH_API)
	}
	routes::route("/", get(handler))
}

fn health_check() -> Router {
	async fn handler(conn: Extension<PgPool>) -> impl IntoResponse {
		let result = sqlx::query!("SELECT 1 as value")
			.fetch_one(&*conn)
			.await
			.unwrap();

		println!("{:?}", result.value);

		tracing::info!("Health check success");
		responder::as_plain(StatusCode::OK, crate::cmd::about())
	}

	routes::route("/health", get(handler))
}
