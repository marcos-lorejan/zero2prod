use axum::{extract::State, Json};
use crate::{db, models::User, state::AppState};


pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> String {
    match db::signup(state.supabase_client, payload).await {
        Ok(_) => "Signup successful!".to_string(),
        Err(message) => message,
    }
}
