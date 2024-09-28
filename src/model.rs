use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

//stores attributes for products
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductAttributes {
    pub name: String,
    pub slug: String,
    pub terms: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
//this is the sqlx database model
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ProductModel {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: String,
    pub price: String,
    pub sku: String,
    pub product_type: String,
    pub stock: String,
    pub allow_backorders: String,
    pub low_stock_threshold: String,
    pub shipping_weight: String,
    pub product_gallery: String,
    pub attributes: String,
    pub variations: String,
    pub shipping_dimensions: String,
    pub shipping_class: String,
    pub tax_status: String,
    pub tax_class: String,
    pub published: Option<bool>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
