use std::env;
use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};

use once_cell::sync::Lazy;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::cors::{Any, CorsLayer};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

mod controllers;
mod models;
mod error;
mod utils;

// keys for JWT token
static KEYS: Lazy<models::auth::Keys> = Lazy::new(|| {
    let secret = dotenv::var("JWT_SECRET").unwrap_or_else(|_|"Your secret here".to_owned());
    models::auth::Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    dotenv().ok();

    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();


    let cors = CorsLayer::new().allow_origin(Any);

    let durl = env::var("DATABASE_URL").expect("set DATABASE_URL env variable");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to connect database");

    let app = Router::new()
        .route("/", get(|| async {"hello, world"}))
        .route("/login", post(controllers::auth::login))
        .route("/register",post(controllers::auth::register))
        // logged in user only 
        .route("/user_profile",get(controllers::user::user_profile))
        .layer(cors)
        .layer(Extension(pool));

    let addr = std::net::SocketAddr::from(([127,0,0,1],3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");

}
