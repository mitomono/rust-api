use actix_web::test::TestRequest;
use actix_web::{test, App};
use dotenv::dotenv;

use libAPI::employees::init_routes;

pub mod init_tests;

#[actix_rt::test]
async fn get_all_users() {
    dotenv().ok();
    let app = test::init_service(App::new().configure(init_routes)).await;

    let resp = TestRequest::get()
        .uri("/employees")
        .send_request(&app)
        .await;
    assert!(resp.status().is_success(), "Failed to find employees");
}

#[actix_rt::test]
async fn get_users() {
    dotenv().ok();
    let app = test::init_service(App::new().configure(init_routes)).await;

    let resp = TestRequest::get()
        .uri("/get?id=1&first_name=adrian")
        .send_request(&app)
        .await;
    assert!(resp.status().is_success(), "Failed to find employees");
}
