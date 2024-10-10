use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use axum::routing::method_routing::get_service;
use tower_http::services::ServeDir;

use crate::{
    handler::{
        create_product_form, health_checker_handler, multipart_create_product_handler, product_attributes_template, single_product_display, tera_product_handler, create_product_attribute_handler, product_terms_template, create_product_terms_handler
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    //Serve the whole frontend folder
    let path = "/frontend";
    //let path = "/frontend/img";
    //let path = "/frontend/static";
    Router::new()
        .route("/multipart_create_product", get(create_product_form).post(multipart_create_product_handler))
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/products", get(tera_product_handler))
        .route("/attributes", get(product_attributes_template).post(create_product_attribute_handler))
        .route("/product/:id", get(single_product_display))
        .route("/product-attribute/:id", get(product_terms_template).post(create_product_terms_handler))
        //.route("/:id", get(single_product_display))
        //.nest_service(path, get_service(ServeDir::new("./frontend/")).handle_error(|_| async { (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error") }))
        .nest_service(path, get_service(ServeDir::new("./frontend/")).handle_error(|_| async { (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Error") }))
        .with_state(app_state)
}
