mod pool;

use ::std::error::Error;
use sqlx::{Error as SqlxError, PgPool};
use uuid::Uuid;

use super::super::Store;
use crate::{
	repository::models::User,
	system_models::{AppError, CoreResult},
};

impl From<SqlxError> for AppError {
	fn from(err: SqlxError) -> Self {
		return AppError::system_error(err);
	}
}

pub(crate) struct PostgresStore {
	pool: PgPool,
}

impl PostgresStore {
	pub(crate) async fn new() -> Result<Self, Box<dyn Error>> {
		let pool = pool::create_db_connection().await?;
		Ok(Self { pool })
	}
}

impl Store for PostgresStore {
	async fn read_user_by_id(&self, user_id: Uuid) -> CoreResult<Option<User>> {
		sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1;")
			.bind(user_id)
			.fetch_optional(&self.pool)
			.await
			.map_err(Into::into)
	}

	async fn close(&self) {
		self.pool.close().await;
	}
}
