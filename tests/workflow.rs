use ::std::{error::Error, sync::Arc};
use axum::http::StatusCode;
use axum_test::TestServer;
use my_axum_template::{repository::Repository, router};

#[tokio::test]
async fn test_workflow() -> Result<(), Box<dyn Error>> {
	let repo = Repository::new().await?;
	let app = router::create_router(Arc::new(repo));
	let server = TestServer::new(app).unwrap();

	// Пустой репозиторий
	let res = server.get("/api/hello").await;
	assert_eq!(res.status_code(), StatusCode::OK);

	Ok(())
}
