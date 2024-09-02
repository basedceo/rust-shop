use std::sync::Arc;
use tera::Tera;
use tera::Context;

use axum::{
    extract::{Path, Query, State, Form},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde_json::json;
use serde::{Deserialize, Serialize};

use crate::{
    model::{NoteModel, ProductModel},
    schema::{CreateProductSchema, CreateNoteSchema, FilterOptions, UpdateNoteSchema},
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
    Ok(Json(json_response))
}


pub async fn note_list_handler(
    //optional parameter to filter results when querying larger databases
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        NoteModel,
        "SELECT * FROM notes ORDER by id LIMIT $1 OFFSET $2",
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

    let notes = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    println!("notes are {:?}", notes);
    //for i in  notes {
    //    //println!("***{:?}", i);
    //    //print_type_of(&i);
    //    println!("content = {:?}", i.content);
    //    print_type_of(&i.content);
    //}
    
    //TODO
    //create a seperate function to display the templated HTML, download tests to verify how those
    //functions should work
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = Context::new();
    for note in  notes {
        context.insert("note.title", &note.title);
        context.insert("note.category", &note.category);
        context.insert("note.content", &note.content);

        let output = tera.render("hello.html", &context);
        Html(output.unwrap());
    }

    Ok(Json(json_response))
}

pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("create_note_handler() triggered");
    println!("This is our body {:?}", body);
    print_type_of(&body);
    println!("Title= {:?}", body.title.to_string());
    print_type_of(&body.title);
    println!("Content= {:?}", body.content.to_string());
    print_type_of(&body.content.to_string());
    println!("Category= {:?}", body.category.to_owned().unwrap_or("".to_string()));
    print_type_of(&body.category.to_owned().unwrap_or("".to_string()));
    println!("Published= {:?}", body.published);
    print_type_of(&body.published);
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});
            println!("this is note_response {:?}", note_response);
            print_type_of(&note_response);

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
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

pub async fn get_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return Ok(Json(note_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn edit_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        NoteModel,
        "UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *",
        body.title.to_owned().unwrap_or(note.title),
        body.content.to_owned().unwrap_or(note.content),
        body.category.to_owned().unwrap_or(note.category.unwrap()),
        body.published.unwrap_or(note.published.unwrap()),
        now,
        id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            return Ok(Json(note_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn delete_note_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM notes  WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Note with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
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

//display submitted form in HTML with tera
pub async fn display_form() {
    let mut tera = Tera::new("templates/**/*").unwrap();
    let mut context = Context::new();
}

//this functions like create_note_handler but with an html form
pub async fn accept_form(
    State(data): State<Arc<AppState>>,
    //this formats `body` the same as Json(body) in create_note_handler
    Form(body): Form<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("accept_form() triggered");
    println!("This is our body {:?}", body);
    print_type_of(&body);
    println!("Title= {:?}", body.title);
    print_type_of(&body.title);
    println!("Content= {:?}", body.content);
    print_type_of(&body.content);
    println!("Category= {:?}", body.category.to_owned().unwrap_or("".to_string()));
    print_type_of(&body.category.to_owned().unwrap_or("".to_string()));
    println!("Published= {:?}", body.published);
    print_type_of(&body.published);
    let query_result = sqlx::query_as!(
        NoteModel,
        "INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *",
        body.title.to_string(),
        body.content.to_string(),
        body.category.to_owned().unwrap_or("".to_string())
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});
            println!("this is note_response {:?}", note_response);
            print_type_of(&note_response);

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
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
            println!("this is note_response {:?}", note_response);
            print_type_of(&note_response);

            return Ok((StatusCode::CREATED, Json(note_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
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
