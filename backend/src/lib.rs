use axum::Router;
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use supabase_rs::SupabaseClient;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use axum::http;

pub mod db;
pub mod handlers;
pub mod models;
pub mod state;

use state::AppState;

pub async fn run(listener: TcpListener) -> Result<(), std::io::Error> {
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

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await
}
