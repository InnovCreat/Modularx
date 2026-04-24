use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Term {
    pub name: String,
    pub ontology: String,
    pub layer: String,
    pub cycle: String,
    pub domain: String,
    pub temporality: String,
    pub abstractness: String,
    pub intent: String,
}

const VALID_ONTOLOGIES: &[&str] = &["Object", "MessageStatic", "MessageDynamic", "Property", "Process"];
const VALID_LAYERS: &[&str] = &["Solid", "Dynamic", "Resonance", "All"];
const VALID_CYCLES: &[&str] = &["Call", "Receive", "Tick", "OpenGate", "Generate", "External", "Evolution", "Transmit"];
const VALID_DOMAINS: &[&str] = &[
    "VeritasHortus", "AlchemCore", "Consciousness", "ObjectBlocks",
    "SecurityEthics", "MathComputation", "NetworksSystems",
    "VisualizationAesthetics", "AllianceCollaboration",
];
const VALID_TEMPORALITIES: &[&str] = &["Static", "Dynamic", "Cyclic", "Emergent"];
const VALID_ABSTRACTNESSES: &[&str] = &["Theoretical", "Functional", "Structural"];
const VALID_INTENTS: &[&str] = &["Harmonic", "Generative", "Protective", "Dissonant"];

pub fn parse_term_tree(path: &str) -> Result<HashMap<String, Term>, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut terms = HashMap::new();

    for (i, line) in content.lines().enumerate() {
        let line = line.trim();

        // Skip empty lines, comments, and structural characters
        if line.is_empty()
            || line.starts_with("//")
            || line.starts_with('#')
            || line.starts_with('─')
            || line.starts_with('├')
            || line.starts_with('└')
            || !line.contains('|')
        {
            continue;
        }

        let parts: Vec<&str> = line.split('|').map(|p| p.trim()).collect();
        if parts.len() != 8 {
            eprintln!("⚠️  Ligne {}: format invalide ({} parties): {}", i + 1, parts.len(), line);
            continue;
        }

        let term = Term {
            name: parts[0].to_string(),
            ontology: parts[1].to_string(),
            layer: parts[2].to_string(),
            cycle: parts[3].to_string(),
            domain: parts[4].to_string(),
            temporality: parts[5].to_string(),
            abstractness: parts[6].to_string(),
            intent: parts[7].to_string(),
        };

        if !VALID_ONTOLOGIES.contains(&term.ontology.as_str()) {
            eprintln!("⚠️  {} : Ontology invalide: '{}'", term.name, term.ontology);
            continue;
        }
        if !VALID_LAYERS.contains(&term.layer.as_str()) {
            eprintln!("⚠️  {} : Layer invalide: '{}'", term.name, term.layer);
            continue;
        }
        if !VALID_CYCLES.contains(&term.cycle.as_str()) {
            eprintln!("⚠️  {} : Cycle invalide: '{}'", term.name, term.cycle);
            continue;
        }
        if !VALID_DOMAINS.contains(&term.domain.as_str()) {
            eprintln!("⚠️  {} : Domain invalide: '{}'", term.name, term.domain);
            continue;
        }
        if !VALID_TEMPORALITIES.contains(&term.temporality.as_str()) {
            eprintln!("⚠️  {} : Temporality invalide: '{}'", term.name, term.temporality);
            continue;
        }
        if !VALID_ABSTRACTNESSES.contains(&term.abstractness.as_str()) {
            eprintln!("⚠️  {} : Abstractness invalide: '{}'", term.name, term.abstractness);
            continue;
        }
        if !VALID_INTENTS.contains(&term.intent.as_str()) {
            eprintln!("⚠️  {} : Intent invalide: '{}'", term.name, term.intent);
            continue;
        }

        terms.insert(term.name.clone(), term);
    }

    println!("✓ {} termes chargés depuis {}", terms.len(), path);
    Ok(terms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tree() {
        let terms = parse_term_tree("static/VERITAS_HORTUS.tree").unwrap();
        assert!(!terms.is_empty());
        assert!(terms.contains_key("LUNA"));
        assert!(terms.contains_key("Call"));
        assert!(terms.contains_key("Harmony"));
    }
}
