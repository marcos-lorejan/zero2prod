use std::sync::Arc;
use supabase_rs::SupabaseClient;
use crate::models::User;


pub async fn signup(supabase_client: Arc<SupabaseClient>, user: User) -> Result<(), String> {
    let user_data = serde_json::to_string(&user).unwrap();
    println!("Attempting to insert user: {}", user_data);

    let result = supabase_client
        .insert(
            "users",
            user_data,
        )
        .await;

    match result {
        Ok(_) => {
            println!("Insert successful!");
            Ok(())
        }
        Err(e) => {
            eprintln!("Insert failed: {}", e);
            Err("Signup failed.".to_string())
        }
    }
}
