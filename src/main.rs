extern crate core;

use std::env;
use std::net::SocketAddr;

use axum::response::Html;
use axum::Router;
use axum::routing::get;
use axum::routing::post;
use dotenvy::dotenv;
use sea_orm::*;
use tracing_subscriber;

use crate::organization_handlers::{create, find};
use crate::user_handlers::{current, login, register};

// todo is it ok???
mod auth_service;
mod db;
mod encryption;
mod entities;
mod jwt;
mod organization_handlers;
mod user_handlers;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tokio::main]
async fn main() {
    // .env file support
    dotenv().ok();

    // logger
    // format configure: https://docs.rs/tracing-subscriber/0.3.16/tracing_subscriber/fmt/index.html
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    // heap profiler
    // doc: https://docs.rs/dhat/latest/dhat/
    // view profile: https://nnethercote.github.io/dh_view/dh_view.html
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let db = db::get_connection().await;
    let state = AppState { conn: db };

    let app = Router::new()
        .route("/", get(handler))
        .route("/find/:id", get(find))
        .route("/create/:name", get(create))
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/current", get(current))
        .with_state(state.clone());

    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse::<u16>()
        .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    println!("Server started on {}", addr);

    server.await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
