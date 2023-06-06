// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

// use axum::extract::State;
use axum::response::{IntoResponse, Redirect};
use axum::{http::StatusCode, routing::*, Router};

use crate::server::BASE_PATH_API;
use crate::{routes, server::responder};
use crate::{PKG_NAME, PKG_VERSION};

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
	async fn handler() -> impl IntoResponse {
		let platform = format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS);
		let timestamp = build_time::build_time_utc!("%Y-%m-%d %H:%M:%S UTC");
		let message = format!("{} {} {} ({})", PKG_NAME, PKG_VERSION, platform, timestamp);
		responder::as_plain(StatusCode::OK, message)
	}

	// async fn handler(State(pool): State<sqlx::PgPool>) -> impl IntoResponse {
	// 	let result = sqlx::query_scalar("select 'hello world from pg'").fetch_one(&pool);

	// 	match result.await {
	// 		Ok(_val) => (StatusCode::OK, String::from("Hahhaha")),
	// 		Err(_) => todo!(),
	// 	}
	// }

	routes::route("/health", get(handler))
}
