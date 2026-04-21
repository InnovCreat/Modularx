use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactStatus {
    Prototype,
    #[serde(rename = "in_progress")]
    InProgress,
    Working,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactType {
    Code,
    Documentation,
    Module,
    Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    #[serde(rename = "artifactId")]
    pub artifact_id: u32,

    pub name:        String,

    #[serde(rename = "createdBy")]
    pub created_by:  String,

    pub timestamp:   u64,
    pub status:      ArtifactStatus,

    #[serde(rename = "type")]
    pub kind:        ArtifactType,

    pub frequency:   Option<String>,

    #[serde(rename = "tickOrder")]
    pub tick_order:  Vec<u32>,

    pub variables:   serde_json::Value,
}
