use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub(crate) struct User {
	pub id: Uuid,
	pub nickname: String,
	pub email: Option<String>,
}
