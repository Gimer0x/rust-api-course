use sqlx::types::chrono;
use bigdecimal::BigDecimal;

use crate::controllers::auth::SignUpRequest;

pub async fn has_user_email(db: &sqlx::MySqlPool, email: &str) -> bool {
    sqlx::query!("SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .unwrap()
        .is_some()
}

pub async fn create_user(db: &sqlx::MySqlPool, user: &SignUpRequest) -> bool {
    let password_hash = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap();

    sqlx::query!("INSERT INTO users (email, password, first_name, last_name) VALUES (?, ?, ?, ?)", 
            &user.email, 
            &password_hash, 
            &user.first_name, 
            &user.last_name
        )
        .execute(db)
        .await
        .is_ok()
}

pub struct User {
    pub id: u64,
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub balance: BigDecimal,
    pub created_at: chrono::NaiveDate,
    pub updated_at: chrono::NaiveDate,
}

pub async fn get_user_by_email(db: &sqlx::MySqlPool, email: &str) -> Option<User> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", email)
        .fetch_optional(db)
        .await
        .unwrap()
}