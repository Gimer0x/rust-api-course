use actix_web::{App, HttpServer, middleware::from_fn, web};
use dotenvy::dotenv;
use tokio::sync::Mutex;

mod controllers;
mod db;
mod middleware;
mod utils;

pub struct AppState {
    db: Mutex<sqlx::MySqlPool>,
    jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let state = web::Data::new(AppState {
        db: Mutex::new(
            sqlx::MySqlPool::connect(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
        ),
        jwt_secret: std::env::var("JWT_SECRET").unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(controllers::auth::sign_up)
            .service(controllers::auth::sign_in)
            .service(
                web::scope("/api")
                    .wrap(from_fn(middleware::auth::verify_jwt))
                    .service(controllers::users::get_profile)
                    .service(controllers::users::update_profile),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
