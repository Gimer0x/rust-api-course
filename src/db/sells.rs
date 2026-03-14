use sqlx::types::chrono::NaiveDateTime;
use bigdecimal::BigDecimal;
use serde::Serialize;

use crate::controllers::sells::{CreateSellRequest, UpdateSellRequest};

#[derive(Serialize)]
pub struct Sell {
    pub id: u64,
    pub user_id: u64,
    pub product_id: u64,
    pub quantity: u32,
    pub price: BigDecimal,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn get_sells_by_user_id(db: &sqlx::MySqlPool, user_id: u64) -> Vec<Sell> {
    sqlx::query_as!(Sell, "SELECT * FROM sells WHERE user_id = ?", user_id)
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn create_sell(db: &sqlx::MySqlPool, user_id: u64, sell: &CreateSellRequest) -> Sell {
    let result = sqlx::query_as!(Sell, "INSERT INTO sells (user_id, product_id, quantity, price, description) VALUES (?, ?, ?, ?, ?)", user_id, sell.product_id, sell.quantity, sell.price, sell.description)
        .execute(db)
        .await
        .unwrap();

    get_sell_by_id(db, result.last_insert_id()).await.unwrap()
}

pub async fn get_sell_by_id(db: &sqlx::MySqlPool, id: u64) -> Option<Sell> {
    sqlx::query_as!(Sell, "SELECT * FROM sells WHERE id = ?", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn update_product_quantity(db: &sqlx::MySqlPool, product_id: u64, quantity: i32) {
    sqlx::query!(
        "UPDATE products SET quantity = quantity - ? WHERE id = ?",
        quantity,
        product_id
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn update_sell(db: &sqlx::MySqlPool, id: u64, sell: &UpdateSellRequest) {
    sqlx::query!(
        "UPDATE sells SET description = ? WHERE id = ?",
        sell.description,
        id
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn delete_sell(db: &sqlx::MySqlPool, id: u64) {
    sqlx::query!("DELETE FROM sells WHERE id = ?", id)
        .execute(db)
        .await
        .unwrap();
}