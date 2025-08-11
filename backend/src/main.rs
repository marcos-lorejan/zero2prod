use axum::{routing::get, Router};
use dotenvy::dotenv;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use supabase_rs::SupabaseClient;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use axum::http;

mod db;
mod handlers;
mod models;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set.");
    let supabase_anon_key = env::var("SUPABASE_ANON_KEY").expect("SUPABASE_ANON_KEY must be set.");
    let supabase_client = Arc::new(SupabaseClient::new(supabase_url, supabase_anon_key).unwrap());

    let app_state = AppState { supabase_client };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(vec![http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/signup", axum::routing::post(handlers::signup))
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
