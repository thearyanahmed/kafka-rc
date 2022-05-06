use crate::spawn_app;

#[tokio::test]
async fn health_check_works() {
	let app = spawn_app().await;

	let route = app.url("api/v1/health-check");

	println!("route: {}",route);

	let client = reqwest::Client::new();

	let response = client
		.get(route)
		.send()
		.await
		.expect("failed to execute test");

	println!("response {}\n {:?}",response.status(),response);

	assert!(response.status().is_success());
}

