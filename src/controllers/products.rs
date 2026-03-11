use actix_web::{Responder, delete, get, post, put};
use crate::{AppState, db, utils};
use actix_web::{HttpRequest, web, HttpResponse};
use serde::Deserialize;
use bigdecimal::BigDecimal;
use serde_json::json;

#[get("/products")]
pub async fn product_index(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.lock().await;
    let products = db::products::get_products_by_user_id(&db, utils::get_user_id(&req))
        .await;

    HttpResponse::Ok().json(products)  
}

#[derive(Deserialize, Debug)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub quantity: i32,
}

#[post("/products")]
pub async fn product_create(state: web::Data<AppState>, req: HttpRequest, data: web::Json<CreateProductRequest>) -> impl Responder {
    let db = state.db.lock().await;
    let user = utils::get_authenticated_user(&req, &db).await;
    let product = db::products::create_product(&db, user.id, &data).await;

    HttpResponse::Ok().json(product)
}

#[get("/products/{id}")]
pub async fn product_show(
        state: web::Data<AppState>,
        req: HttpRequest,
        path: web::Path<u64>,
    ) -> impl Responder {
    let id = path.into_inner();
    let db = state.db.lock().await;
    let user = utils::get_authenticated_user(&req, &db).await;

    let Some(product) = db::products::get_product_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Product not found"
        }));
    };

    if product.user_id != user.id {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "You are not allowed to access this product"
        }));
    }

    HttpResponse::Ok().json(product)
}

#[derive(Deserialize, Debug)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<BigDecimal>,
    pub quantity: Option<i32>,
}

#[put("/products/{id}")]
pub async fn product_update(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>, data: web::Json<UpdateProductRequest>) -> impl Responder {
    let id = path.into_inner();
    let db = state.db.lock().await;
    let user = utils::get_authenticated_user(&req, &db).await;

    let Some(product) = db::products::get_product_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Product not found"
        }));
    };

    if product.user_id != user.id {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "You are not allowed to update this product"
        }));
    }

    db::products::update_product(&db, id, &data).await;
    let product = db::products::get_product_by_id(&db, id).await;


    HttpResponse::Ok().json(product)
}

#[delete("/products/{id}")]
pub async fn product_delete(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>) -> impl Responder {
    let db = state.db.lock().await;
    let user = utils::get_authenticated_user(&req, &db).await;
    let id = path.into_inner();

    let Some(product) = db::products::get_product_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Product not found"
        }));
    };

    if product.user_id != user.id {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "You are not allowed to delete this product"
        }));
    }

    db::products::delete_product(&db, id).await;

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Product deleted successfully"
    }))
}

