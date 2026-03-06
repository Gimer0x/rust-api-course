use actix_web::{post, web, Responder, HttpResponse};
use serde::{Deserialize};
use crate::{AppState, db::users};
use serde_json::json;

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

#[post("/auth/sign-in")]
pub async fn sign_in() -> impl Responder {
    "Sign In"
}