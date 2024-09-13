use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_product_form, create_product_handler, health_checker_handler, index, product_list_handler, tera_product_handler, get_file_upload, file_upload_handler
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/create_product", get(create_product_form).post(create_product_handler))
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/products",get(product_list_handler))
        .route("/products", get(tera_product_handler))
        .route("/upload", get(get_file_upload).post(file_upload_handler))
        .with_state(app_state)
}
