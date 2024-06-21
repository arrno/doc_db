use crate::tree::node::Node;
use axum::{
    extract::Path,
    extract::State,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

struct AppState {
    data: Mutex<Node<String>>,
}

#[tokio::main]
pub async fn serve() {
    let root = Node::new(Some(String::from("root")));
    let state = Arc::new(AppState {
        data: Mutex::new(root),
    });
    let app = Router::new()
        .route("/document", get(query_documents))
        .route("/document", post(create_document))
        .route("/document/:label", get(get_document))
        .route("/document/:label", patch(update_document))
        .route("/document/:label", delete(delete_document))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_document(
    State(state): State<Arc<AppState>>,
    Path(path): Path<String>,
) -> (StatusCode, Json<Option<Document>>) {
    let state = Arc::clone(&state);
    let mut root = state.data.lock().unwrap();
    let path_vec: Vec<&str> = path.split(".").collect();
    let result = root.check(&path_vec);
    let doc = match result {
        Some(node_info) => Document {
            label: node_info.label,
            value: match node_info.value {
                Some(val) => Some(val.to_string()),
                None => None,
            },
        },
        None => return (StatusCode::NOT_FOUND, Json(None)),
    };
    (StatusCode::OK, Json(Some(doc)))
}

async fn query_documents(Path(path): Path<String>) -> (StatusCode, Json<Vec<Document>>) {
    let doc = Document {
        label: "example".to_string(),
        value: None,
    };
    (StatusCode::OK, Json(vec![doc]))
}

async fn create_document(Json(payload): Json<DocumentPayload>) -> (StatusCode, Json<Document>) {
    let doc = Document {
        label: "example".to_string(),
        value: None,
    };
    (StatusCode::CREATED, Json(doc))
}

async fn update_document(Json(payload): Json<DocumentPayload>) -> (StatusCode, Json<Document>) {
    let doc = Document {
        label: "example".to_string(),
        value: None,
    };
    (StatusCode::OK, Json(doc))
}

async fn delete_document(Path(path): Path<String>) -> (StatusCode, Json<Document>) {
    let doc = Document {
        label: "example".to_string(),
        value: None,
    };
    (StatusCode::OK, Json(doc))
}

#[derive(Serialize, Deserialize)]
struct Document {
    label: String,
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct DocumentPayload {
    path: Vec<String>,
    data: Document,
}
