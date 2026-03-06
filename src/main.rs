use actix_web::{App, web, HttpServer};
use tokio::sync::Mutex;
use dotenvy::dotenv;

mod controllers;
mod db;


pub struct AppState {
    db: Mutex<sqlx::MySqlPool>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let state = web::Data::new(AppState {
        db: Mutex::new(
            sqlx::MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap()
        )
    });
    HttpServer::new(move || App::new()
        .app_data(state.clone())
        .service(controllers::auth::sign_up)
        .service(controllers::auth::sign_in)
        .service(controllers::users::get_profile)
        .service(controllers::users::update_profile)
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}