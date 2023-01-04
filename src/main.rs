extern crate diesel;

// #[macro_use]
// extern crate diesel_migrations;

use std::env;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;

mod books;
mod db;
mod error_handler;
mod members;
mod schema;
mod swagger;
pub mod utils;

fn set_routes(config: &mut web::ServiceConfig) {
    swagger::init_swagger(config);
    members::init_routes(config);
    books::init_routes(config);
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || App::new().configure(set_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{host}:{port}"))?
        }
    };

    server.run().await
}
