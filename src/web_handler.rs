use crate::trie::node::Node;
use axum::{
    extract::Query,
    extract::State,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
        .route("/document", get(get_document))
        .route("/document", post(create_document))
        .route("/document", patch(update_document))
        .route("/document", delete(delete_document))
        .route("/query/document", get(query_documents))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn get_document(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> (StatusCode, Json<Option<Document>>) {
    let state = Arc::clone(&state);
    let root = state.data.lock().unwrap();
    let path = match params.get("path") {
        Some(val) => val,
        None => return (StatusCode::BAD_REQUEST, Json(None)),
    };
    let vec_path = &path.split(".").collect::<Vec<&str>>();
    let doc = match root.check(&vec_path) {
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

async fn query_documents(
    State(state): State<Arc<AppState>>,
    Query(params): Query<QueryParams>,
) -> (StatusCode, Json<Vec<Document>>) {
    let state = Arc::clone(&state);
    let root = state.data.lock().unwrap();
    let vec_path = &params.path.split(".").collect::<Vec<&str>>();
    let param_op = match ExpOp::from_str(&params.op) {
        Some(op) => op,
        None => return (StatusCode::BAD_REQUEST, Json(vec![])),
    };
    let results = root.query(&vec_path, |doc| match doc {
        Some(val) => match param_op {
            ExpOp::Eq => val == &params.target,
            ExpOp::Neq => val != &params.target,
            ExpOp::St => val.starts_with(&params.target),
            ExpOp::Ctn => val.contains(&params.target),
        },
        None => false,
    });
    (
        StatusCode::OK,
        Json(
            results
                .iter()
                .map(|doc| Document {
                    label: doc.label.to_string(),
                    value: match doc.value {
                        Some(v) => Some(v.to_string()),
                        None => None,
                    },
                })
                .collect(),
        ),
    )
}

async fn create_document(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DocumentPayload>,
) -> StatusCode {
    let state = Arc::clone(&state);
    let mut root = state.data.lock().unwrap();
    let vec_path = &payload.path.split(".").collect::<Vec<&str>>();
    root.insert(&vec_path, payload.value);
    StatusCode::CREATED
}

async fn update_document(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<DocumentPayload>,
) -> (StatusCode, Json<Option<Document>>) {
    let state = Arc::clone(&state);
    let mut root = state.data.lock().unwrap();
    let mut result_val = None;
    let vec_path = payload.path.split(".").collect::<Vec<&str>>();
    if let Some(val) = payload.value {
        result_val = match root.update(&vec_path, val.into()) {
            Ok(res) => Some(res.clone()),
            Err(_) => return (StatusCode::BAD_REQUEST, Json(None)),
        };
    }
    let doc = match vec_path.len() {
        0 => None,
        n => Some(Document {
            label: vec_path[n - 1].to_string(),
            value: result_val,
        }),
    };
    (StatusCode::OK, Json(doc))
}

async fn delete_document(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> StatusCode {
    let path = match params.get("path") {
        Some(val) => val,
        None => return StatusCode::BAD_REQUEST,
    };
    let state = Arc::clone(&state);
    let mut root = state.data.lock().unwrap();
    let vec_path = &path.split(".").collect::<Vec<&str>>();
    match root.delete(&vec_path) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}

#[derive(Serialize, Deserialize)]
struct QueryParams {
    pub op: String,
    pub path: String,
    pub target: String,
}

#[derive(Serialize, Deserialize)]
enum ExpOp {
    Eq,
    Neq,
    St,
    Ctn,
}

impl ExpOp {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "Eq" => Some(Self::Eq),
            "Neq" => Some(Self::Neq),
            "St" => Some(Self::St),
            "Ctn" => Some(Self::Ctn),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Document {
    label: String,
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct DocumentPayload {
    path: String,
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct DocQuery {
    path: String,
    target: String,
    op: ExpOp,
}
