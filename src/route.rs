use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use axum::routing::method_routing::get_service;
use tower_http::services::ServeDir;

use crate::{
    handler::{
        create_product_form, health_checker_handler, index, product_list_handler, tera_product_handler, get_file_upload, file_upload_handler, multipart_create_product_handler
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let path = "/frontend/img";
    Router::new()
        //.route("/create_product", get(create_product_form).post(create_product_handler))
        .route("/multipart_create_product", get(create_product_form).post(multipart_create_product_handler))
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/products",get(product_list_handler))
        .route("/products", get(tera_product_handler))
        //.nest_service(path, ServeDir::new("./static")).handle_error(|_| async { (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error") })
        .nest_service(path, get_service(ServeDir::new("./frontend/img/")).handle_error(|_| async { (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error") }))
        .route("/upload", get(get_file_upload).post(file_upload_handler))
        .with_state(app_state)
}
