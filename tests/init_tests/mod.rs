extern crate libAPI;

use actix_web::{App, HttpServer};
use std::env;

use dotenv::dotenv;
use libAPI::employees;
use listenfd::ListenFd;

pub async fn init_server() -> std::io::Result<()> {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes));

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
