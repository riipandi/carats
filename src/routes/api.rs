// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use axum::response::IntoResponse;
use axum::{routing::*, Json, Router};

use crate::server::responder;
use crate::{routes, server::middleware};

pub fn register() -> Router {
	Router::new()
		.layer(middleware::cors())
		.merge(api_root())
		.nest("/auth", auth_routes::register())
		.nest("/users", user_routes::register())
		.fallback(responder::throw_not_found)
}

pub fn api_root() -> Router {
	async fn handler() -> impl IntoResponse {
		let message = format!("This is default {} API endpoint", crate::PKG_NAME);
		Json(serde_json::json!({ "message": message }))
	}
	routes::route("/", get(handler))
}

mod auth_routes {
	use crate::handler::auth as handler;
	use crate::routes::route;
	use axum::{routing::*, Router};

	pub fn register() -> Router {
		Router::new()
			.merge(route("/login", get(handler::login_index)))
			.merge(route("/login", post(handler::post_login)))
			.merge(route("/register", get(handler::signup_index)))
	}
}

mod user_routes {
	use crate::handler::user as handler;
	use crate::routes::route;
	use axum::{routing::*, Router};

	pub fn register() -> Router {
		Router::new().merge(route("/", get(handler::index)))
	}
}
