use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
	body::Body,
	http::Request,
	middleware::Next,
	response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::system_models::{AppError, AppResponse};

pub(super) async fn auth_middleware(mut req: Request<Body>, next: Next) -> Response {
	let Ok(now) = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.map(|dur| dur.as_secs())
	else {
		return AppResponse::system_error("Time went backwards", None).into_response();
	};

	if now % 2 != 0 {
		return AppError::SessionExpired.into_response();
	}

	req.extensions_mut().insert(Uuid::nil());

	next.run(req).await
}

pub(super) async fn optional_auth_middleware(mut req: Request<Body>, next: Next) -> Response {
	let Ok(now) = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.map(|dur| dur.as_secs())
	else {
		return AppError::unauthorized("Time went backwards").into_response();
	};

	let user_id = match now % 2 {
		0 => Some(Uuid::nil()),
		_ => None,
	};

	req.extensions_mut().insert(user_id);

	next.run(req).await
}
