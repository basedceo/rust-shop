use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

//stores attributes for products
#[derive(Eq, Hash, PartialEq, Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct ProductAttributes {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub order_by: String,
    //TODO turn this into a vector of string arrays Vec<[T; N]>
    //pub terms: Vec<[String; 3]>, // New field: vector of arrays, each with 3 strings
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

//stores categories for products
#[derive(Eq, Hash, PartialEq, Clone, Debug, FromRow, Deserialize, Serialize)]
pub struct ProductCategories {
    pub lvl: String,
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub parent: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub display_type: String,
    pub thumbnail: String,
    pub count: i32, // Count of products in this category
    //pub child_categories: Option<Vec<Uuid>>, // Array of UUIDs for child categories
    pub child_categories: Option<Vec<JsonValue>>, // JSONB array of child category info
    //TODO turn this into a vector of string arrays Vec<[T; N]>
    //pub terms: Vec<[String; 3]>, // New field: vector of arrays, each with 3 strings
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct ProductTerms {
    pub product_id: Uuid,       // Foreign key to ProductAttributes
    pub name: String,         // term Name First term (NOT NULL)
    pub slug: String,         // term Slug Second term (NOT NULL)
    pub description: Option<String>, // term Description Third term (can be NULL)
}

//TODO create a mapping of ProduductTerms to Product Attributes

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
