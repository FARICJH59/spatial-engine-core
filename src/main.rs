mod parser;
mod crdt;

use axum::{
    routing::{get, post},
    Json, Router,
};
use tower_http::cors::CorsLayer;
use std::sync::{Arc, Mutex};
use crdt::{SpatialCRDTStore, SceneNodeCRDT};

#[derive(Clone)]
struct AppState {
    crdt_store: Arc<Mutex<SpatialCRDTStore>>,
}

#[tokio::main]
async fn main() {
    println!("Starting Spatial Engine CRDT Synchronized Server on port 3000...");

    let state = AppState {
        crdt_store: Arc::new(Mutex::new(SpatialCRDTStore::new())),
    };

    let state_get = state.clone();
    let state_post = state.clone();

    let app = Router::new()
        .route("/api/scene", get(move || async move {
            let store = state_get.crdt_store.lock().unwrap();
            Json(store.get_all())
        }))
        .route("/api/sync", post(move |Json(payload): Json<SceneNodeCRDT>| async move {
            // Validate scale via Hoare logic parser before accepting CRDT sync
            if let Ok(()) = parser::validate_scale(payload.scale.x, payload.scale.y, payload.scale.z) {
                let mut store = state_post.crdt_store.lock().unwrap();
                let accepted = store.merge(payload.clone());
                Json(serde_json::json!({ 
                    "status": "success", 
                    "crdt_accepted": accepted, 
                    "node": payload 
                }))
            } else {
                Json(serde_json::json!({ "status": "error", "message": "Hoare validation failed: invalid scale dimensions" }))
            }
        }))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("CRDT Spatial Engine API live at http://127.0.0.1:3000/api/scene");
    
    axum::serve(listener, app).await.unwrap();
}
