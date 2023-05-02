use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
        health, todos_list_handler,
    },
    model,
};

pub fn create_router() -> Router {
    let db = model::todo_db();

    Router::new()
        .route("/api/health", get(health))
        // Without parameter
        .route(
            "/api/todos",
            post(create_todo_handler).get(todos_list_handler),
        )
        // With parameter
        .route(
            "/api/todos/:id",
            get(get_todo_handler)
                .patch(edit_todo_handler)
                .delete(delete_todo_handler),
        )
        // Inject state for in-memory with Arc<Mutex<?>> purpose
        .with_state(db)
}
