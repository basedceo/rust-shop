use std::sync::Arc;
use tera::{to_value, Tera};
use lazy_static::lazy_static;
use tera::Context;
use tokio::fs;
use base64::prelude::*;
use uuid::Uuid;


use axum::{
    extract::{multipart, Form, Multipart, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde_json::json;
use serde::{Deserialize, Serialize};

use crate::{
    model::{ProductAttributes, ProductCategories, ProductModel, ProductTerms},
    schema::{CreateProductSchema, FilterOptions},
    AppState,
};

pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Simple CRUD API with Rust, SQLX, Postgres,and Axum";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}

//TODO fix so that products[0].title can be inserted into the page name
fn common_context() -> tera::Context {
    let mut context = tera::Context::new();
    context.insert("title", "Based.win");
    context
}

//Displays products with tera templates at 127.0.0.1:8000/products
//TODO rename this to featured_products_handler
//TODO fix this function and all other tera functions so that they don't crash when the db is
//empty, static images should still be able to render, information from db should be renderend in
//an if else
pub async fn tera_product_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Html<String> {
    let Query(opts) = opts.unwrap_or_default();

    //the amount of products that will be listed in featured products
    let limit = opts.limit.unwrap_or(14);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching featured products",
        });
        //TODO create function to handle errors
        //return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let products = query_result.unwrap();

    //tera
    let tera = Tera::new("frontend/**/*.html").unwrap();
    let mut context = common_context();

    context.insert("page_title", "Index");
    context.insert("message", "This is Index page.");
    context.insert("products", &products);

    //Static images used across most pages
    let static_images = vec!["frontend/static/logo_small.webp", "frontend/static/button.png"];
    context.insert("static_img", &static_images);

    // Specify the uploads folder
    let uploads_dir = std::path::Path::new("frontend/img/products");

    // Vector to store the image paths
    let mut images = Vec::new(); 

    //Read the contents of the uploads directory
    //TODO Turn this into a function that can be called when needed
    if let Ok(entries) = std::fs::read_dir(uploads_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    // Only include files with valid image extensions (e.g., jpg, png)
                    if ext =="jpg" || ext == "png" || ext == "jpeg" || ext == "gif" || ext == "webp" {
                        if let Some(path_str) = path.to_str() {
                            images.push(path_str.to_string());
                            println!("image path is {}", path_str.to_string());

                        }
                    }
                }
            }
        }
    }
    
    context.insert("images", &images);

    let output = tera.render("index.html", &context);
    Html(output.unwrap())
}

//TODO rename this to single product template
pub async fn single_product_display(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>,
) -> Html<String> {

    //TODO turn this query into a function
    let query_result = sqlx::query_as!(
        ProductModel,
        "SELECT * FROM products WHERE id = $1",
        id as Uuid,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
        //TODO create function to handle errors
        //return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let products = query_result.unwrap();
    println!("*****SINGLE PRODUCT PAGE*****");

    //tera
    let tera = Tera::new("frontend/**/*.html").unwrap();
    let mut context = common_context();

    context.insert("page_title", &products[0].title);

    context.insert("products", &products);

    let static_images = vec!["frontend/static/logo_small.webp", "frontend/static/button.png"];
    //let static_images = vec!["frontend/img/logo_small.webp", "frontend/img/button.png"];
    context.insert("static_img", &static_images);

    let output = tera.render("single_product.html", &context);
    Html(output.unwrap())

}

pub async fn product_attributes_template(
    State(data): State<Arc<AppState>>
) -> Html<String> {
    //TODO make this into a function that can be reused in product catalog and single product


    let query_result = sqlx::query_as!(
        ProductAttributes,
        "SELECT * FROM product_attributes ORDER by id",
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all product attribute items",
        });
        //TODO create function to handle errors
        //return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    //TODO you probably only need to select the term names
    let terms_query = sqlx::query_as!(
        ProductTerms,
        "SELECT * FROM product_terms ORDER by product_id"
    )
    .fetch_all(&data.db)
    .await;

    if terms_query.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all product attribute items",
        });
        //TODO create function to handle errors
        //return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let attributes = query_result.unwrap();
    let terms = terms_query.unwrap();

    //tera
    let tera = Tera::new("frontend/**/*.html").unwrap();
    let mut context = common_context();

    context.insert("page_title", "Product Attributes Page");
    context.insert("attributes", &attributes);
    context.insert("terms", &terms);

    //Static images used across most pages
    let static_images = vec!["frontend/static/logo_small.webp", "frontend/static/button.png"];
    context.insert("static_img", &static_images);

    let output = tera.render("product_attributes/product_attributes.html", &context);
    Html(output.unwrap())
}

pub async fn product_terms_template(
    Path(id): Path<Uuid>,
    State(data): State<Arc<AppState>>
) -> Html<String> {

    //TODO turn this query into a function
    let terms_query = sqlx::query_as!(
        ProductTerms,
        "SELECT * FROM product_terms WHERE product_id = $1",
        id as Uuid,
    )
    .fetch_all(&data.db)
    .await;

    if terms_query.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all product attribute items",
        });
        //TODO create function to handle errors
        //return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let attributes_query = sqlx::query_as!(
        ProductAttributes,
        "SELECT * FROM product_attributes WHERE id = $1",
        id as Uuid,
    )
    .fetch_all(&data.db)
    .await;

    if attributes_query.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching product terms items",
        });
        //TODO create function to handle errors
        //return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let terms = terms_query.unwrap();
    let attributes = attributes_query.unwrap();
    
    //tera
    //let tera = Tera::new("frontend/**/*.html").unwrap();
    //let tera = Tera::new("frontend/**/*.html,frontend/product_attributes/**/*.html").unwrap();
    let tera = Tera::new("frontend/**/*.html").unwrap();
    //let tera = Tera::new("frontend/*.html,frontend/**/*.html").unwrap();
    //let tera = Tera::new("frontend/product_attributes/**/*.html").unwrap();
    let mut context = common_context();

    context.insert("page_title", "Product Attributes Page");
    context.insert("terms", &terms);
    context.insert("attributes", &attributes);

    //Static images used across most pages
    let static_images = vec!["frontend/static/logo_small.webp", "frontend/static/button.png"];
    context.insert("static_img", &static_images);

    //let output = tera.render("product_app_control_attribute_edit.html", &context);
    let output = tera.render("product_attributes/product_app_control_attribute_edit.html", &context);

    Html(output.unwrap())
}

//TODO finish implementing this function, based on product_attributes_template
pub async fn product_categories_template(
    State(data): State<Arc<AppState>>
) -> Html<String> {
    //TODO make this into a function that can be reused in product catalog and single product


    let query_result = sqlx::query_as!(
        ProductCategories,
        "SELECT * FROM product_categories ORDER by id",
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all product attribute items",
        });
        //TODO create function to handle errors
        //return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let categories = query_result.unwrap();

    //tera
    let tera = Tera::new("frontend/**/*.html").unwrap();
    let mut context = common_context();

    context.insert("page_title", "Product Categories Page");
    context.insert("categories", &categories);

    //Static images used across most pages
    let static_images = vec!["frontend/static/logo_small.webp", "frontend/static/button.png"];
    context.insert("static_img", &static_images);

    let output = tera.render("product_categories/product_categories.html", &context);
    Html(output.unwrap())
}

pub async fn create_product_terms_handler(
    State(data): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut term_name = String::new();
    let mut term_slug = String::new();
    let mut term_desc = String::new();
    let mut attribute_id = Uuid::new_v4();


    // Iterate over multipart fields to collect name, slug, and terms
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            match field_name {
                "attribute_id" => {
                    let id_str = field.text().await.unwrap();
                    attribute_id = Uuid::parse_str(&id_str).unwrap();
                }
                "term_name" => {
                    term_name = field.text().await.unwrap();
                }
                "term_slug" => {
                    term_slug = field.text().await.unwrap();
                }
                "term_desc" => {
                    term_desc = field.text().await.unwrap();
                }
                _ => {
                    println!("Unexpected field: {}", field_name);
                }
            }
        }
    }

    // Validate that required fields are populated
    if term_name.is_empty() || term_slug.is_empty() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Name and slug are required fields.",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    // Now insert into the database after fields are collected
    println!("inserting product term into the database");
    let query_result = sqlx::query_as!(
        ProductTerms,
        "INSERT INTO product_terms (product_id, name, slug, description) VALUES ($1, $2, $3, $4) RETURNING *",
        attribute_id,
        term_name,
        term_slug,
        term_desc,
    )
    .fetch_one(&data.db)
    .await;

    // Handle the result of the database operation
    //TODO verify if this is what prints in the browser and update it accordingly
    match query_result {
        Ok(attribute) => {
            let response = json!({
                "status": "success",
                "data": {
                    "attribute": attribute
                }
            });
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Product attribute with that name already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn create_product_form() -> Html<&'static str> {
    Html(std::include_str!("../create_product_form.html"))
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub async fn create_attribute_form() -> Html<&'static str> {
    Html(std::include_str!("../frontend/product_attributes/product_attributes.html"))
}

//Create product attributes
//TODO rename to create_attribute_handler
pub async fn create_product_attribute_handler(
    State(data): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut attribute_name = String::new();
    let mut attribute_slug = String::new();
    let mut order_by = String::new();

    // Iterate over multipart fields to collect name, slug, and terms
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            match field_name {
                "attribute_name" => {
                    attribute_name = field.text().await.unwrap();
                }
                "attribute_slug" => {
                    attribute_slug = field.text().await.unwrap();
                }
                "order_by" => {
                    order_by = field.text().await.unwrap();
                }
                _ => {
                    println!("Unexpected field: {}", field_name);
                }
            }
        }
    }

    // Validate that required fields are populated
    if attribute_name.is_empty() || attribute_slug.is_empty() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Name, slug, and terms are required fields.",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    // Now insert into the database after fields are collected
    let query_result = sqlx::query_as!(
        ProductAttributes,
        "INSERT INTO product_attributes (name, slug, order_by) VALUES ($1, $2, $3) RETURNING *",
        attribute_name,
        attribute_slug,
        order_by,
    )
    .fetch_one(&data.db)
    .await;

    // Handle the result of the database operation
    match query_result {
        Ok(attribute) => {
            let response = json!({
                "status": "success",
                "data": {
                    "attribute": attribute
                }
            });
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Product attribute with that name already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error", "message": format!("{:?}", e)})),
            ));
        }
    }
}

////TODO get a function working that can accept product parameters and images
////TODO images with a space in the name aren't being displayed properly, the src="" ends at the space
///Some of this was written by ChatGPT
///creates products
pub async fn multipart_create_product_handler(
    State(data): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut text_inputs = vec![String::new(); 17]; // Store text inputs

    //maps variables to tag names in create_product_form.html
    let field_mapping = [
        "title", "description", "category", "price", "sku", "product_type", "stock",
        "allow_backorders", "low_stock_threshold", "shipping_weight", "product_gallery", "attributes",
        "variations", "shipping_dimensions", "shipping_class", "tax_status", "tax_class"
    ];
    
    //TODO Remove this, it's unnecessary
    //tera
    //let tera = Tera::new("frontend/**/*.html").unwrap();
    //let mut context = common_context();

    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(field_name) = field.name() {
            print!("{:?} = ", field_name);
            //TODO refactor this so that the product_gallery case is handled first and its related
            //text_inputs[index] is equal to the file path after the image is uploaded
            //if let Some(index) = field_mapping.iter().position(|&name| name == field_name) {
            if field_name == "product_gallery" {
                // File upload handling
                let file_name = field.file_name().unwrap().to_string();
                let content_type = field.content_type().unwrap().to_string();
                let data = field.bytes().await.unwrap();

                let Some(file_type) = content_type.split('/').nth(1) else {
                    return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid file type"}))));
                };

                //let upload_path = format!("/home/kenny/code/Rust/rust-axum-postgres-api/uploads/{}", file_name);
                let upload_path = format!("frontend/img/products/{}", BASE64_STANDARD.encode(&file_name));
                println!("Uploading file to {:?}", upload_path);
                fs::write(&upload_path, data).await.unwrap();
                text_inputs[10] = upload_path;
            } else if let Some(index) = field_mapping.iter().position(|&name| name == field_name) {
                text_inputs[index] = field.text().await.unwrap();
                //println!("{:?} is {:?}", field_name, text_inputs[index]);
                println!("{:?}", text_inputs[index]);
            } 
        }
    }

    let query_result = sqlx::query_as!(
        ProductModel,
        "INSERT INTO products (
        title,
        description,
        category,
        price,
        sku,
        product_type,
        stock,
        allow_backorders,
        low_stock_threshold,
        shipping_weight,
        product_gallery,
        attributes,
        variations,
        shipping_dimensions,
        shipping_class,
        tax_status,
        tax_class) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) RETURNING *",
        text_inputs[0],
        text_inputs[1],
        text_inputs[2],
        text_inputs[3],
        text_inputs[4],
        text_inputs[5],
        text_inputs[6],
        text_inputs[7],
        text_inputs[8],
        text_inputs[9],
        text_inputs[10],
        text_inputs[11],
        text_inputs[12],
        text_inputs[13],
        text_inputs[14],
        text_inputs[15],
        text_inputs[16]
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Product with that name already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
    // Process `text_inputs` as necessary
    
    //Ok(())
}
