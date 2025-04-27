mod implementations;
pub(crate) mod models;

use ::std::error::Error;
use implementations::PostgresStore;
use models::User;
use uuid::Uuid;

use crate::system_models::CoreResult;

trait Store {
	async fn read_user_by_id(&self, user_id: Uuid) -> CoreResult<Option<User>>;

	async fn close(&self);
}

pub struct Repository {
	store: PostgresStore,
}

impl Repository {
	pub async fn new() -> Result<Self, Box<dyn Error>> {
		return Ok(Self {
			store: PostgresStore::new().await?,
		});
	}

	pub(crate) async fn read_user_by_id(&self, user_id: Uuid) -> CoreResult<Option<User>> {
		self.store.read_user_by_id(user_id).await
	}

	pub async fn close(&self) {
		return self.store.close().await;
	}
}
