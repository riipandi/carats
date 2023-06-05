// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn login_index() -> impl IntoResponse {
	let body = Json(json!({
	  "message": "Not yet implemented"
	}));
	(StatusCode::OK, body)
}

pub async fn post_login() -> impl IntoResponse {
	let body = Json(json!({
	  "message": "Not yet implemented"
	}));
	(StatusCode::OK, body)
}
