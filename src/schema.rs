use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

//stores attributes for products
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductAttributes {
    pub name: String,
    pub slug: String,
    //pub terms: Vec<String>,
    //pub terms: Vec<[String; 3]>,
    pub order_by: String,
}

//stores categories for products
#[derive(Serialize, Deserialize, Debug)]
pub struct ProductCategories {
    pub parent: String,
    pub name: String,
    pub slug: String,
    //pub terms: Vec<String>,
    //pub terms: Vec<[String; 3]>,
    pub order_by: String,
    pub description: String,
    pub display_type: String,
    pub thumbnail: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductTerms {
    pub name: String,         // First term (NOT NULL)
    pub slug: String,         // Second term (NOT NULL)
    pub description: Option<String>, // Third term (can be NULL)
}

//TODO add the following parameters
//featured
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProductSchema {
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
    //TODO break this up into length width and height
    pub shipping_dimensions: String,
    pub shipping_class: String,
    //this is optional
    pub tax_status: String,
    pub tax_class: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}
