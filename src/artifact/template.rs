use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateContent {
    pub code:   Option<String>,
    pub rust:   Option<String>,
    pub doc:    Option<String>,
    pub schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id:       u32,
    pub name:     String,
    pub category: String,
    pub version:  String,
    pub content:  TemplateContent,
}

pub type TemplateCatalogue = HashMap<u32, Template>;

pub fn catalogue_from_json(json: &str) -> Result<TemplateCatalogue, serde_json::Error> {
    let templates: Vec<Template> = serde_json::from_str(json)?;
    Ok(templates.into_iter().map(|t| (t.id, t)).collect())
}
