mod parser;

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneNode {
    pub object_id: String,
    pub object_category: String,
    pub object_caption: String,
    pub supported_by: String,
    pub position: Vector3D,
    pub scale: Vector3D,
    pub orientation: Vector3D,
    pub lighting_state: String,
}

#[derive(Clone)]
struct AppState {
    nodes: Arc<Mutex<Vec<SceneNode>>>,
}

#[tokio::main]
async fn main() {
    println!("Starting Spatial Engine Authoritative Server on port 3000...");

    let initial_node = SceneNode {
        object_id: "lantern_001".to_string(),
        object_category: "Lantern".to_string(),
        object_caption: "A traditional red lantern with intricate carvings".to_string(),
        supported_by: "FLOOR".to_string(),
        position: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
        scale: Vector3D { x: 1.0, y: 1.0, z: 1.0 },
        orientation: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
        lighting_state: "Day".to_string(),
    };

    let state = AppState {
        nodes: Arc::new(Mutex::new(vec![initial_node])),
    };

    let state_get = state.clone();
    let state_post = state.clone();

    let app = Router::new()
        .route("/api/scene", get(move || async move {
            let nodes = state_get.nodes.lock().unwrap();
            Json(nodes.clone())
        }))
        .route("/api/spawn", post(move |Json(payload): Json<SceneNode>| async move {
            if let Ok(()) = parser::validate_scale(payload.scale.x, payload.scale.y, payload.scale.z) {
                let mut nodes = state_post.nodes.lock().unwrap();
                nodes.push(payload.clone());
                Json(serde_json::json!({ "status": "success", "node": payload }))
            } else {
                Json(serde_json::json!({ "status": "error", "message": "Invalid scale dimensions" }))
            }
        }))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Spatial Engine API live at http://127.0.0.1:3000/api/scene");
    
    axum::serve(listener, app).await.unwrap();
}
