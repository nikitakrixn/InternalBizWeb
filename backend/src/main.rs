use axum::{
    extract::Path,
    Router,
    routing::{delete, get},
    serve,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // set up our application with "hello world" route at "/
    let app = Router::new()
        .route("/hello/:visitor", get(greet_visitor))
        .route("/bye", delete(say_goodbye));

    // start the server on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

/// Extract the `visitor` path parameter and use it to greet the visitor.
async fn greet_visitor(Path(visitor): Path<String>) -> String {
    format!("Hello, {visitor}!")
}

/// Say goodbye to the visitor.
async fn say_goodbye() -> String {
    "Goodbye".to_string()
}