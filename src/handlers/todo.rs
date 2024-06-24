use axum::{Router, routing::get, response::Html};

pub fn routes() -> Router {
    Router::new()
        .route("/", get(todo_list))
}

async fn todo_list() -> Html<&'static str> {
    Html("<h1>Todo List</h1>")
}