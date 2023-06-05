// Copyright 2023-current Aris Ripandi <aris@duck.com>
// SPDX-License-Identifier: MIT OR Apache-2.0

use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::server::responder::JsonResponse;

pub async fn index() -> impl IntoResponse {
	let body = Json(JsonResponse {
		status_code: StatusCode::OK.as_u16(),
		message: Some("List all users"),
		data: Some(vec!["one", "two", "three"]),
	});
	(StatusCode::OK, body)
}
