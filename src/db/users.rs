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