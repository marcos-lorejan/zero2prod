use std::sync::Arc;
use supabase_rs::SupabaseClient;

#[derive(Clone)]
pub struct AppState {
    pub supabase_client: Arc<SupabaseClient>,
}
