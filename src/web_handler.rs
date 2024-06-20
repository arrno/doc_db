use axum::{
    routing::{get, post, patch, delete},
    http::StatusCode,
    extract::Path,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn serve() {
    let app = Router::new()
        .route("/document", get(query_documents))
        .route("/document", post(create_document))
        .route("/document/:label", get(get_document))
        .route("/document/:label", patch(update_document))
        .route("/document/:label", delete(delete_document));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_document(Path(path): Path<String>) -> (StatusCode, Json<Document>) {
    let doc = Document{
        label: "example".to_string(),
        value: "example".to_string(),
    };
    (StatusCode::OK, Json(doc))
}

async fn query_documents(Path(path): Path<String>) -> (StatusCode, Json<Vec<Document>>) {
    let doc = Document{
        label: "example".to_string(),
        value: "example".to_string(),
    };
    (StatusCode::OK, Json(vec![doc]))
}

async fn create_document(Json(payload): Json<DocumentPayload>) -> (StatusCode, Json<Document>) {
    let doc = Document{
        label: "example".to_string(),
        value: "example".to_string(),
    };
    (StatusCode::CREATED, Json(doc))
}

async fn update_document(Json(payload): Json<DocumentPayload>) -> (StatusCode, Json<Document>) {
    let doc = Document{
        label: "example".to_string(),
        value: "example".to_string(),
    };
    (StatusCode::OK, Json(doc))
}

async fn delete_document(Path(path): Path<String>) -> (StatusCode, Json<Document>) {
    let doc = Document{
        label: "example".to_string(),
        value: "example".to_string(),
    };
    (StatusCode::OK, Json(doc))
}

#[derive(Serialize, Deserialize)]
struct Document {
    label: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct DocumentPayload {
    path: Vec<String>,
    data: Document,
}