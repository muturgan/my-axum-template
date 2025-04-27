use ::std::sync::Arc;
use axum::{
	Router, middleware,
	routing::{get, post},
};

use crate::{auth, handlers as H, repository::Repository};

pub fn create_router(repo: Arc<Repository>) -> Router {
	let router = Router::new()
		.nest(
			"/api",
			Router::new()
				.route("/hello", get(H::hello_world))
				.merge(
					Router::new()
						.route("/reg/hello", post(H::for_mayby_logined))
						.layer(middleware::from_fn(auth::optional_auth_middleware)),
				)
				.merge(
					Router::new()
						.route("/reg/hello", post(H::for_logined))
						.layer(middleware::from_fn(auth::auth_middleware)),
				),
		)
		.with_state(repo);

	return router;
}
