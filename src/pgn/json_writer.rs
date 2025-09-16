//! JSON writer for chess repertoire
//! Serializes the repertoire tree to JSON format.

use crate::{domain::RepertoireNode, pgn::RepertoireWriter};
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
struct JsonNode {
    id: u64,
    parent: Option<u64>,
    children: Vec<u64>,
    // Add other fields as needed
}

/// JSON writer that serializes the repertoire tree to JSON format.
#[derive(Default)]
pub struct JsonWriter;

impl RepertoireWriter for JsonWriter {
    fn write(&self, root: &RepertoireNode) -> Result<String> {
        let json_root = JsonNode {
            id: root.id,
            parent: root.parent,
            children: root.children.iter().map(|c| c.id).collect(),
            // Add other fields as needed
        };
        serde_json::to_string(&json_root).map_err(Into::into)
    }
}
