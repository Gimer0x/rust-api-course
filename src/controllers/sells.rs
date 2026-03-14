use actix_web::{get, post, put, delete, Responder};
use crate::{AppState, db, utils};
use actix_web::{HttpRequest, web, HttpResponse};
use serde::{Deserialize};
use bigdecimal::BigDecimal;
use serde_json::json;

#[get("/sells")]
pub async fn sell_index(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let db = state.db.lock().await;
    let sells = db::sells::get_sells_by_user_id(&db, utils::get_user_id(&req))
        .await;

    HttpResponse::Ok().json(sells)
}

#[derive(Deserialize, Debug)]
pub struct CreateSellRequest {
    pub product_id: u64,
    pub quantity: i32,
    pub price: BigDecimal,
    pub description: Option<String>,
}

#[post("/sells")]
pub async fn sell_create(
        state: web::Data<AppState>, 
        req: HttpRequest, data: web::Json<CreateSellRequest>
    ) -> impl Responder {

    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);

    let Some(product) = db::products::get_product_by_id(&db, data.product_id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Product not found"
        }));
    };

    if product.quantity < data.quantity {
        return HttpResponse::BadRequest().json(json!({
            "status": "error",
            "message": "Not enough product in stock"
        }));
    }

    if product.user_id != user_id {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "You are not allowed to sell this product"
        }));
    }

    let sell = db::sells::create_sell(&db, user_id, &data).await;

    db::sells::update_product_quantity(&db, data.product_id, data.quantity).await;

    HttpResponse::Ok().json(sell)
}


#[get("/sells/{id}")]
pub async fn sell_show(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let id = path.into_inner();

    let Some(sell) = db::sells::get_sell_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Sell not found"
        }));
    };

    if sell.user_id != user_id {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "You are not allowed to access this sell"
        }));
    }

    HttpResponse::Ok().json(sell)
}

#[derive(Deserialize)]
pub struct UpdateSellRequest {
    pub description: Option<String>,
}

#[put("/sells/{id}")]
pub async fn sell_update(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>, 
    data: web::Json<UpdateSellRequest>) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let id = path.into_inner();

    let Some(sell) = db::sells::get_sell_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Sell not found"
        }));
    };

    if sell.user_id != user_id {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "You are not allowed to update this sell"
        }));
    }

    db::sells::update_sell(&db, id, &data).await;

    let sell = db::sells::get_sell_by_id(&db, id).await;

    HttpResponse::Ok().json(sell)
}

#[delete("/sells/{id}")]
pub async fn sell_delete(state: web::Data<AppState>, req: HttpRequest, path: web::Path<u64>) -> impl Responder {
    let db = state.db.lock().await;
    let user_id = utils::get_user_id(&req);
    let id = path.into_inner();

    let Some(sell) = db::sells::get_sell_by_id(&db, id).await else {
        return HttpResponse::NotFound().json(json!({
            "status": "error",
            "message": "Sell not found"
        }));
    };

    if sell.user_id != user_id {
        return HttpResponse::Forbidden().json(json!({
            "status": "error",
            "message": "You are not allowed to delete this sell"
        }));
    }
    db::sells::update_product_quantity(&db, sell.product_id, -(sell.quantity as i32)).await; 

    db::sells::delete_sell(&db, id).await;
       

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Sell deleted successfully"
    }))
}