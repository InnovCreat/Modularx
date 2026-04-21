use crate::artifact::{
    artifact::Artifact,
    template::{Template, TemplateCatalogue},
    views::{ArtifactViews, CodeView, TreeNode, TreeView, TextView},
};

#[derive(Debug)]
pub enum TickResult {
    Ok  { views: ArtifactViews },
    Err { failed_at: usize, template_id: u32, reason: String },
}

/// Moteur de tick — Fixed Sacred Order.
/// Résout chaque template dans l'ordre exact de artifact.tick_order.
/// Un seul template manquant → Err avec position et ID précis.
pub fn tick(artifact: &Artifact, catalogue: &TemplateCatalogue) -> TickResult {
    let mut resolved: Vec<&Template> = Vec::with_capacity(artifact.tick_order.len());

    for (i, &id) in artifact.tick_order.iter().enumerate() {
        match catalogue.get(&id) {
            Some(tmpl) => resolved.push(tmpl),
            None => return TickResult::Err {
                failed_at:   i,
                template_id: id,
                reason:      format!("Template {} not found in catalogue", id),
            },
        }
    }

    TickResult::Ok { views: build_views(artifact, &resolved) }
}

fn build_views(artifact: &Artifact, templates: &[&Template]) -> ArtifactViews {
    // Code view — fragments Rust dans l'ordre du tick
    let code = templates
        .iter()
        .filter_map(|t| t.content.rust.as_deref())
        .collect::<Vec<_>>()
        .join("\n\n// ---\n\n");

    // Text view — docs dans l'ordre du tick
    let explanation = templates
        .iter()
        .filter_map(|t| t.content.doc.as_deref())
        .collect::<Vec<_>>()
        .join(" ");

    // Tree view — hiérarchie plate (les enfants sont les templates résolus)
    let children = templates
        .iter()
        .map(|t| TreeNode {
            template_id:   t.id,
            template_name: t.name.clone(),
            children:      vec![],
        })
        .collect();

    ArtifactViews {
        code: CodeView {
            artifact_id: artifact.artifact_id,
            name:        artifact.name.clone(),
            code,
        },
        tree: TreeView {
            artifact_id: artifact.artifact_id,
            name:        artifact.name.clone(),
            root: TreeNode {
                template_id:   0,
                template_name: artifact.name.clone(),
                children,
            },
        },
        text: TextView {
            artifact_id: artifact.artifact_id,
            name:        artifact.name.clone(),
            explanation,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifact::{artifact::{Artifact, ArtifactStatus, ArtifactType}, template::Template};
    use std::collections::HashMap;

    fn make_template(id: u32, name: &str) -> Template {
        Template {
            id,
            name:     name.to_string(),
            category: "Object Data".to_string(),
            version:  "1.0.0".to_string(),
            content:  crate::artifact::template::TemplateContent {
                code:   None,
                rust:   Some(format!("// {}", name)),
                doc:    Some(format!("Doc for {}", name)),
                schema: None,
            },
        }
    }

    fn make_artifact(tick_order: Vec<u32>) -> Artifact {
        Artifact {
            artifact_id: 999,
            name:        "Test Artifact".to_string(),
            created_by:  "test".to_string(),
            timestamp:   0,
            status:      ArtifactStatus::Prototype,
            kind:        ArtifactType::Module,
            frequency:   None,
            tick_order,
            variables:   serde_json::Value::Null,
        }
    }

    #[test]
    fn tick_ok_resolves_all_templates() {
        let mut catalogue = HashMap::new();
        catalogue.insert(1, make_template(1, "Core Philosophy"));
        catalogue.insert(2, make_template(2, "Structure"));

        let artifact = make_artifact(vec![1, 2]);
        match tick(&artifact, &catalogue) {
            TickResult::Ok { views } => {
                assert_eq!(views.tree.root.children.len(), 2);
                assert!(views.code.code.contains("Core Philosophy"));
            }
            TickResult::Err { .. } => panic!("expected Ok"),
        }
    }

    #[test]
    fn tick_err_identifies_missing_template() {
        let catalogue: HashMap<u32, Template> = HashMap::new();
        let artifact = make_artifact(vec![1, 99]);

        match tick(&artifact, &catalogue) {
            TickResult::Err { failed_at, template_id, .. } => {
                assert_eq!(failed_at, 0);
                assert_eq!(template_id, 1);
            }
            TickResult::Ok { .. } => panic!("expected Err"),
        }
    }

    #[test]
    fn tick_err_reports_correct_position() {
        let mut catalogue = HashMap::new();
        catalogue.insert(1, make_template(1, "Core Philosophy"));
        // template 67 manquant intentionnellement

        let artifact = make_artifact(vec![1, 67]);
        match tick(&artifact, &catalogue) {
            TickResult::Err { failed_at, template_id, .. } => {
                assert_eq!(failed_at, 1);      // position 1 dans le tick_order
                assert_eq!(template_id, 67);   // template 67 manquant
            }
            TickResult::Ok { .. } => panic!("expected Err"),
        }
    }
}
