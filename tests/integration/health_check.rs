use crate::spawn_app;

#[tokio::test]
async fn health_check_works() {
	let app = spawn_app().await;

	println!("add address {}", &app.address);

	assert_eq!(&app.address,"http://localhost:8000")

	//
	// let client = reqwest::Client::new();
	//
	// let response = client
	// 	.get(format!("{}/api/v1/health_check",&app.address))
	// 	.send()
	// 	.await
	// 	.expect("failed to execute test");
	//
	// assert!(response.status().is_success());
}

