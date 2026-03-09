use crate::db;
use actix_web::{HttpMessage, HttpRequest};

pub fn get_user_id(req: &HttpRequest) -> u64 {
    let ext = req.extensions();

    ext.get::<u64>().unwrap().to_owned()
}

pub async fn get_authenticated_user(req: &HttpRequest, db: &sqlx::MySqlPool) -> db::users::User {
    db::users::get_user_by_id(db, get_user_id(req))
        .await
        .unwrap()
}
