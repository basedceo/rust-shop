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
    pub terms: Vec<String>,
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
