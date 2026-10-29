use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneNodeCRDT {
    pub object_id: String,
    pub object_category: String,
    pub object_caption: String,
    pub supported_by: String,
    pub position: Vector3D,
    pub scale: Vector3D,
    pub orientation: Vector3D,
    pub lighting_state: String,
    pub timestamp: u64, // Logical/Physical timestamp for LWW resolution
    pub client_id: String,
}

pub struct SpatialCRDTStore {
    nodes: HashMap<String, SceneNodeCRDT>,
}

impl SpatialCRDTStore {
    pub fn new() -> Self {
        let mut store = Self { nodes: HashMap::new() };
        // Insert default baseline entity
        let genesis_node = SceneNodeCRDT {
            object_id: "lantern_001".to_string(),
            object_category: "Lantern".to_string(),
            object_caption: "Genesis authoritative lantern".to_string(),
            supported_by: "FLOOR".to_string(),
            position: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            scale: Vector3D { x: 1.0, y: 1.0, z: 1.0 },
            orientation: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            lighting_state: "Day".to_string(),
            timestamp: current_timestamp(),
            client_id: "authoritative_server".to_string(),
        };
        store.nodes.insert(genesis_node.object_id.clone(), genesis_node);
        store
    }

    /// Merge incoming node update using Last-Write-Wins (LWW) CRDT rule
    pub fn merge(&mut self, incoming: SceneNodeCRDT) -> bool {
        if let Some(existing) = self.nodes.get(&incoming.object_id) {
            if incoming.timestamp >= existing.timestamp {
                self.nodes.insert(incoming.object_id.clone(), incoming);
                return true; // State updated
            }
            false // Rejected older state
        } else {
            self.nodes.insert(incoming.object_id.clone(), incoming);
            true // Inserted new node
        }
    }

    pub fn get_all(&self) -> Vec<SceneNodeCRDT> {
        self.nodes.values().cloned().collect()
    }
}

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}
