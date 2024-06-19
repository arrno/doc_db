use axum::{routing::get, Router};

#[tokio::main]
async fn serve() {
    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    String::from("Hello, World!")
}
