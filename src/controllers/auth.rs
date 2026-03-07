use actix_web::{post, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use bcrypt::verify;
use std::time::SystemTime;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::{AppState, db::users};

#[derive(Deserialize, Debug)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

#[post("/auth/sign-up")]
pub async fn sign_up( 
    state: web::Data<AppState>, 
    data: web::Json<SignUpRequest>
) -> impl Responder {
    let db = state.db.lock().await;
    if users::has_user_email(&db, &data.email).await {
        return HttpResponse::UnprocessableEntity().json(
            json!({
                "status": "error",
                "message": "Email already exists"
            })
        );
    }

    users::create_user(&db, &data).await;

    HttpResponse::Ok().json(
        json!({
            "status": "success",
            "message": "User created successfully"
        })
    )
}

#[derive(Deserialize, Debug)]
pub struct SignInRequest {
	pub email: String,
	pub password: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: u64,
    pub role: String,
    pub exp: u64,
}

#[post("/auth/sign-in")]
pub async fn sign_in(
    state: web::Data<AppState>, 
    data: web::Json<SignInRequest>
) -> impl Responder {
    let db = state.db.lock().await;

    let Some(user) = users::get_user_by_email(&db, &data.email).await else{
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid email or password"
        }));
    };

    if !verify(&data.password, &user.password).unwrap() {
        return HttpResponse::Unauthorized().json(json!({
            "status": "error",
            "message": "Invalid email or password"
        }));
    };



    let claims = Claims {
        sub: user.id,
        role: "user".to_string(),
        exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() + 4 * 60 * 60
    };

    let token = encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(state.jwt_secret.as_bytes())
    )
    .unwrap();

    HttpResponse::Ok().json(json!({
        "status": "success",
        "token": token
    }))
}