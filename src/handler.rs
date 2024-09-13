use std::sync::Arc;
use tera::{to_value, Tera};
use lazy_static::lazy_static;
use tera::Context;
use tokio::fs;

use axum::{
    extract::{multipart, Form, Multipart, Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde_json::json;
use serde::{Deserialize, Serialize};

use crate::{
    //model::{NoteModel, ProductModel},
    model::{ProductModel},
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

//File upload
pub async fn file_upload_handler(mut multipart: Multipart) {
    println!("running file_upload_handler");
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        //new var
        let file_name = field.file_name().unwrap().to_string();
        //new var
        let content_type = field.content_type().unwrap().to_string();

        //gets the file extension
        //TODO this dosn't work with the following file extensions
        //.svg .xcf, .txt, 
        //these image extensions work
        //.jpg, .png, .avif, .webp, .jxl, .bmp
        let Some(file_type) = content_type.split('/').nth(1) else {
            //TODO handle this better, most likely by rejecting the upload
            //and asking the user to try again
            panic!("that didn't work");
        };
        println!("filetype is {}", file_type);

        //raw bytes of our upload
        let data = field.bytes().await.unwrap();

        println!(
            "Length of `{name}` (`{file_name}`: `{content_type}`) is {} bytes",
            data.len()
        );

        //writes the file to the /uploaded/ directory
        let path = format!("/home/kenny/code/Rust/rust-axum-postgres-api/uploads/{}", file_name);
        fs::write(path, data).await;
    }
}

pub async fn get_file_upload() -> Html<&'static str> {
    Html(std::include_str!("../file_upload_form.html"))
}

fn common_context() -> tera::Context {
    let mut context = tera::Context::new();
    context.insert("title", "axum-tera");
    context
}

pub async fn tera_product_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Html<String> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
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
            "message": "Something bad happened while fetching all note items",
        });
        //TODO create function to handle errors
        //return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let products = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": products.len(),
        "products": products
    });

    //tera
    let tera = Tera::new("frontend/**/*").unwrap();
    let mut context = common_context();

    context.insert("page_title", "Index");
    context.insert("message", "This is Index page.");
    context.insert("products", &products);
    let output = tera.render("index.html", &context);
    Html(output.unwrap())
}


pub async fn product_list_handler(
    //optional parameter to filter results when querying larger databases
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
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
            "message": "Something bad happened while fetching all note items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let products = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": products.len(),
        "products": products
    });

    println!("products are {:?}", products);
    for i in  products {
      //println!("***{:?}", i);
      //print_type_of(&i);
        println!("content = {:?}", i);
        println!("id is {:?}", i.id);
        println!("category is {:?}", i.category);
        print_type_of(&i);
    }

    Ok(Json(json_response))
}


// Display our HTML form for input
pub async fn index() -> Html<&'static str> {
    Html(std::include_str!("../input_form.html"))
}

pub async fn create_product_form() -> Html<&'static str> {
    Html(std::include_str!("../create_product_form.html"))
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(dead_code)]
pub struct Input {
    title: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    published: Option<bool>,
}


pub async fn create_product_handler(
    State(data): State<Arc<AppState>>,
    Form(body): Form<CreateProductSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
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
        body.title.to_string(),
        body.description.to_string(),
        body.category.to_string(),
        body.price.to_string(),
        body.sku.to_string(),
        body.product_type.to_string(),
        body.stock.to_string(),
        body.allow_backorders.to_string(),
        body.low_stock_threshold.to_string(),
        body.shipping_weight.to_string(),
        body.product_gallery.to_string(),
        body.attributes.to_string(),
        body.variations.to_string(),
        body.shipping_dimensions.to_string(),
        body.shipping_class.to_string(),
        body.tax_status.to_string(),
        body.tax_class.to_string()
        //body.tax_class.to_owned().unwrap_or("".to_string()),
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
}
