#[macro_use]
extern crate actix_web;

use std::{env, io};

use actix_web::{middleware, App, HttpServer};

mod response;
mod todo;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enabling logger
            .wrap(middleware::Logger::default())
            // registering services
            .service(todo::index)
            .service(todo::store)
    })
    .bind(("127.0.0.1", 3500))?
    .run()
    .await
}