use bigdecimal::BigDecimal;
use sqlx::types::chrono::NaiveDateTime;
use serde::Serialize;

use crate::controllers::products::{CreateProductRequest, UpdateProductRequest};

#[derive(Serialize)]
pub struct Product {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn get_products_by_user_id(db: &sqlx::MySqlPool, user_id: u64) -> Vec<Product> {
    sqlx::query_as!(Product, "SELECT * FROM products WHERE user_id = ?", user_id)
        .fetch_all(db)
        .await
        .unwrap()
}

pub async fn create_product(db: &sqlx::MySqlPool, user_id: u64, product: &CreateProductRequest) -> Option<Product> {
    let result = sqlx::query!(
        "INSERT INTO products (user_id, name, description, price, quantity) VALUES (?, ?, ?, ?, ?)",
        user_id,
        product.name,
        product.description,
        product.price,
        product.quantity
    )
    .execute(db)
    .await
    .unwrap();

    get_product_by_id(db, result.last_insert_id()).await
}

pub async fn get_product_by_id(db: &sqlx::MySqlPool, id: u64) -> Option<Product> {
    sqlx::query_as!(Product, "SELECT * FROM products WHERE id = ?", id)
        .fetch_optional(db)
        .await
        .unwrap()
}

pub async fn update_product(db: &sqlx::MySqlPool, id: u64, product: &UpdateProductRequest) {
    sqlx::query!(
        "UPDATE products SET name = ?, description = ?, price = ?, quantity = ? WHERE id = ?",
        product.name,
        product.description,
        product.price,
        product.quantity,
        id
    )
    .execute(db)
    .await
    .unwrap();
}

pub async fn delete_product(db: &sqlx::MySqlPool, id: u64) -> bool {
    sqlx::query!("DELETE FROM products WHERE id = ?", id)
        .execute(db)
        .await
        .is_ok()
}