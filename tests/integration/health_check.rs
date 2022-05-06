use crate::spawn_app;

#[tokio::test]
async fn health_check_works() {
	let app = spawn_app().await;

	let route = app.url("/v1/status/health-check");

	println!("route: {}",route);

	let client = reqwest::Client::new();

	let response = client
		.get(route)
		.header("Content-Type","application/json")
		.send()
		.await
		.expect("failed to execute test");

	assert!(response.status().is_success());
}

