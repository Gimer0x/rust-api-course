use actix_web::{get, post, Responder};

#[get("/user")]
pub async fn get_profile() -> impl Responder {
    "Profile"
}

#[post("/user")]
pub async fn update_profile() -> impl Responder {
    "Update Profile"
}