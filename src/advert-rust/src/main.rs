mod common;
mod modules;

use actix_web::middleware::{Compress, Logger};
use actix_web::{get, App, HttpServer, Responder};

use crate::common::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    common::init();
    let state = AppState::new().await.unwrap();

    let app = HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(index)
            .configure(modules::route)
    });

    app.bind("127.0.0.1:8080")?.run().await
}

#[get("/")]
async fn index() -> impl Responder {
    "advert-rust"
}
