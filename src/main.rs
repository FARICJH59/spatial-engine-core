mod parser;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SceneNode {
    pub object_id: String,
    pub object_category: String,
    pub object_caption: String,
    pub supported_by: String,
    pub position: Vector3D,
    pub scale: Vector3D,
    pub orientation: Vector3D,
}

fn main() {
    println!("Initializing Spatial Engine Core Parser...");

    // Sample spatial JSON payload simulating LLM scene generation output
    let sample_json = r#"{
        "object_id": "lantern_001",
        "object_category": "Lantern",
        "object_caption": "A traditional red lantern with intricate carvings",
        "supported_by": "FLOOR",
        "position": { "x": 0.0, "y": 0.0, "z": 0.0 },
        "scale": { "x": 1.0, "y": 1.0, "z": 1.0 },
        "orientation": { "x": 0.0, "y": 0.0, "z": 0.0 }
    }"#;

    match serde_json::from_str::<SceneNode>(sample_json) {
        Ok(node) => {
            println!("Successfully parsed entity: {}", node.object_id);
            // Validate scale directly using the parser module namespace
            match parser::validate_scale(node.scale.x, node.scale.y, node.scale.z) {
                Ok(()) => println!("Scale validation passed: dimensions are valid."),
                Err(e) => eprintln!("Scale validation failed: {}", e),
            }
        }
        Err(err) => eprintln!("Failed to parse spatial JSON payload: {}", err),
    }
}
