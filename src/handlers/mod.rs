use ::std::sync::Arc;
use axum::{
	Extension,
	extract::State,
	response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{
	dto::Dto,
	repository::Repository,
	system_models::{AppResponse, AppResult},
};

#[derive(serde::Deserialize)]
pub struct FakeBody {}

pub(super) async fn hello_world() -> AppResult {
	return Ok(AppResponse::scenario_success("Hello world!", None));
}

pub(super) async fn for_mayby_logined(
	State(_repo): State<Arc<Repository>>,
	Extension(user_id): Extension<Option<Uuid>>,
	Dto(_body): Dto<FakeBody>,
) -> Response {
	AppResponse::scenario_success(format!("your userId id is {user_id:?}"), None).into_response()
}

pub(super) async fn for_logined(
	State(repo): State<Arc<Repository>>,
	Extension(user_id): Extension<Uuid>,
	Dto(_body): Dto<FakeBody>,
) -> AppResult {
	let _ = repo.read_user_by_id(user_id).await?;

	Ok(AppResponse::scenario_success(
		format!("your userId id is {user_id}"),
		None,
	))
}
