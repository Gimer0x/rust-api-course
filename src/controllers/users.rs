use crate::{AppState, db, utils};
use actix_web::{HttpRequest, HttpResponse, Responder, get, post, web};
use serde::Deserialize;

#[get("/users")]
pub async fn get_profile(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.lock().await;
    let user = db::users::get_user_by_id(&db, utils::get_user_id(&req))
        .await
        .unwrap();

    HttpResponse::Ok().json(user)
}

#[derive(Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub first_name: String,
    pub last_name: String,
}

#[post("/users")]
pub async fn update_profile(
    state: web::Data<AppState>,
    req: HttpRequest,
    data: web::Json<UpdateProfileRequest>,
) -> impl Responder {
    let db = state.db.lock().await;
    let user = utils::get_authenticated_user(&req, &db).await;

    db::users::update_user(&db, user.id, &data).await;

    let user = utils::get_authenticated_user(&req, &db).await;

    HttpResponse::Ok().json(user)
}
