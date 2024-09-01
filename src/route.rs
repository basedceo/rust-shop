use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        accept_form, create_note_handler, create_product_form, create_product_handler, delete_note_handler, edit_note_handler, get_note_handler, health_checker_handler, index, note_list_handler, product_list_handler
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(index).post(accept_form))
        .route("/create_product", get(create_product_form).post(create_product_handler))
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/notes/", post(create_note_handler))
        .route("/api/notes", get(note_list_handler))
        .route("/api/products", get(product_list_handler))
        .route(
            "/api/notes/:id",
            get(get_note_handler)
                .patch(edit_note_handler)
                .delete(delete_note_handler),
        )
        .with_state(app_state)
}
