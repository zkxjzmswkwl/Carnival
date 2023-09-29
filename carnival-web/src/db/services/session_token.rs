use easy_password::bcrypt::hash_password;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_session_token(
    remote_addr: &str,
    user_id: i32,
    pool: &SqlitePool
) -> Option<String> {

    let uuid = Uuid::new_v4();
    let hashed_addr = hash_password(remote_addr, uuid.as_bytes(), 12).unwrap();
    println!("{:#?}", hashed_addr);
    // This looks shit. I think move **all** queries to queries.rs. That makes more sense anyway.
    let insert_result = sqlx::query("INSERT INTO session_tokens 
                (for_user, remote_addr, unique_hmac_key, token, is_valid)
                VALUES ($1, $2, $3, $4, $5);")
     .bind(user_id)
     .bind(remote_addr)
     .bind(uuid.to_string())
     .bind(&hashed_addr)
     .bind(true)
     .execute(pool)
     .await;

    match insert_result {
        Ok(_) => Some(hashed_addr),
        Err(e) => {
            eprintln!("{e}");
            None
        }
    }
}
