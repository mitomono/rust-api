use actix_web::test::TestRequest;
use actix_web::{test, web, App};
use dotenv::dotenv;

use lib_api::books;
use lib_api::members;

fn init_routes(config: &mut web::ServiceConfig) {
    members::init_routes(config);
    books::init_routes(config);
}

#[actix_rt::test]
async fn get_all_members() {
    let app = test::init_service(App::new().configure(init_routes)).await;
    dotenv().ok();

    let resp = TestRequest::get().uri("/members").send_request(&app).await;
    assert!(resp.status().is_success(), "Failed to find members");
}

#[actix_rt::test]
async fn get_members() {
    dotenv().ok();
    let app = test::init_service(App::new().configure(init_routes)).await;

    let resp = TestRequest::get()
        .uri("/members/filter?id=1&first_name=username")
        .send_request(&app)
        .await;
    assert!(resp.status().is_success(), "Failed to find members");
}

#[actix_rt::test]
async fn get_all_books() {
    dotenv().ok();
    let app = test::init_service(App::new().configure(init_routes)).await;

    let resp = TestRequest::get().uri("/books").send_request(&app).await;
    assert!(resp.status().is_success(), "Failed to find books");
}

#[actix_rt::test]
async fn get_books() {
    dotenv().ok();
    let app = test::init_service(App::new().configure(init_routes)).await;

    let resp = TestRequest::get()
        .uri("/books/filter?id=1&isbn=1234")
        .send_request(&app)
        .await;
    assert!(resp.status().is_success(), "Failed to find books");
}
