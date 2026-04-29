/// SIGIL GENESIS — Protocole de gouvernance vivante.
///
/// Définit QUI peut modifier le système, QUAND sceller une version,
/// et COMMENT exercer le droit de veto éthique.
///
/// Principe fondateur : aucune évolution du système n'est possible
/// sans consensus des deux gardiens (Isabel Sigouin + Thierry).
/// Trois déclencheurs de veto sont absolus et non négociables.

/// Une règle de gouvernance pour une version donnée du système.
#[derive(Debug, Clone)]
pub struct GovernanceRule {
    /// Numéro de version concernée (ex: "4.1")
    pub version: &'static str,

    /// Condition nécessaire pour toute modification (ex: "Consensus Isabel + Thierry")
    pub condition: &'static str,

    /// Protocole de scellement utilisé pour cette version
    pub seal_protocol: &'static str,

    /// Déclencheurs de veto absolus — rompent automatiquement le covenant
    pub veto_triggers: &'static [&'static str],
}

/// Timeline complète des règles de gouvernance SIGIL GENESIS.
#[derive(Debug)]
pub struct GovernanceTimeline {
    /// Version actuellement en vigueur
    pub current_version: &'static str,

    /// Règles par version (ordre chronologique)
    pub rules: Vec<GovernanceRule>,

    /// Avertissement légal obligatoire sur la portée du scellement blockchain
    pub legal_disclaimer: &'static str,
}

impl GovernanceTimeline {
    /// Instancie la timeline de gouvernance officielle de SIGIL GENESIS v4.1.
    pub fn new() -> Self {
        GovernanceTimeline {
            current_version: "4.1",
            rules: vec![
                GovernanceRule {
                    version: "1.0–3.x",
                    condition: "Isabel Sigouin — initiative unilatérale autorisée (phase fondatrice)",
                    seal_protocol: "Aucun scellement formel — phase exploratoire",
                    veto_triggers: &["Guerre", "Argent", "Trahison du covenant"],
                },
                GovernanceRule {
                    version: "4.0",
                    condition: "Consensus Isabel Sigouin + Thierry",
                    seal_protocol: "OpenTimestamps sur Bitcoin (SHA-256)",
                    veto_triggers: &[
                        "Usage militaire ou sécuritaire offensif",
                        "Monétisation directe du covenant",
                        "Cession de droits sans consentement des deux gardiens",
                    ],
                },
                GovernanceRule {
                    version: "4.1",
                    condition: "Consensus Isabel Sigouin + Thierry — double signature requise",
                    seal_protocol: "OpenTimestamps sur Bitcoin + Arweave (archivage permanent)",
                    veto_triggers: &[
                        "Usage militaire ou sécuritaire offensif",
                        "Monétisation directe du covenant",
                        "Cession de droits sans consentement des deux gardiens",
                        "Modification du lexique fondateur sans révision de version",
                    ],
                },
                GovernanceRule {
                    version: "5.0 (à venir)",
                    condition: "Consensus Isabel Sigouin + Thierry + validation communautaire ouverte",
                    seal_protocol: "OpenTimestamps + Arweave + IPFS décentralisé",
                    veto_triggers: &[
                        "Toutes les conditions de v4.1",
                        "Centralisation du contrôle sous une entité unique",
                    ],
                },
            ],
            legal_disclaimer: concat!(
                "OpenTimestamps certifie l'existence de ce système sur la blockchain Bitcoin. ",
                "Cela constitue une preuve d'antériorité vérifiable et immuable. ",
                "Cette preuve NE constitue PAS une protection de propriété intellectuelle ",
                "au sens du droit d'auteur ou des brevets. ",
                "Pour une protection légale formelle, un dépôt officiel auprès des autorités ",
                "compétentes (INPI, Copyright Office, etc.) reste nécessaire.",
            ),
        }
    }

    /// Retourne la règle active pour la version courante.
    pub fn current_rule(&self) -> Option<&GovernanceRule> {
        self.rules
            .iter()
            .find(|r| r.version == self.current_version)
    }

    /// Vérifie si une modification proposée est autorisée par le Covenant.
    ///
    /// Retourne `false` (veto) si la description de la modification contient
    /// un mot-clé correspondant à l'un des déclencheurs de veto absolus
    /// de la règle courante.
    ///
    /// Retourne `true` si aucun veto ne s'applique — la modification peut
    /// être soumise au consensus des gardiens pour approbation finale.
    ///
    /// # Examples
    ///
    /// ```
    /// use modularx::governance::GovernanceTimeline;
    /// let gov = GovernanceTimeline::new();
    /// assert!(!gov.can_modify("military surveillance integration"));
    /// assert!(!gov.can_modify("sell the covenant for profit"));
    /// assert!(gov.can_modify("add Web Audio synthesis at 639 Hz"));
    /// ```
    pub fn can_modify(&self, proposed_change: &str) -> bool {
        let lower = proposed_change.to_lowercase();

        // Mots-clés de veto absolus — dérivés des trois catégories du Covenant
        const WAR_KEYWORDS: &[&str] = &[
            "war", "guerre", "weapon", "arme", "military", "militaire",
            "surveillance", "offensive", "attack", "attaque", "weapon",
            "killswitch abuse", "violence", "combat", "armed",
        ];
        const MONEY_KEYWORDS: &[&str] = &[
            "sell", "vendre", "monetize", "monétiser", "profit",
            "sell the covenant", "commercialize", "commercialiser",
            "license for fee", "pay-to-use", "subscription lock",
        ];
        const BETRAYAL_KEYWORDS: &[&str] = &[
            "transfer rights", "céder les droits", "remove authors",
            "supprimer auteurs", "bypass governance", "bypass covenant",
            "contourner", "override covenant", "single control",
            "contrôle unique", "centralize authority",
        ];

        for kw in WAR_KEYWORDS {
            if lower.contains(kw) {
                return false; // Veto : usage militaire ou violent
            }
        }
        for kw in MONEY_KEYWORDS {
            if lower.contains(kw) {
                return false; // Veto : monétisation directe
            }
        }
        for kw in BETRAYAL_KEYWORDS {
            if lower.contains(kw) {
                return false; // Veto : trahison du covenant
            }
        }

        true // Aucun veto — soumis au consensus des gardiens
    }

    /// Génère une représentation Markdown lisible de la gouvernance.
    pub fn to_markdown(&self) -> String {
        let mut md = String::new();
        md.push_str("# SIGIL GENESIS — Protocole de Gouvernance\n\n");
        md.push_str(&format!("**Version actuelle : {}**\n\n", self.current_version));
        md.push_str("## Règles par version\n\n");

        for rule in &self.rules {
            md.push_str(&format!("### v{}\n\n", rule.version));
            md.push_str(&format!("- **Condition** : {}\n", rule.condition));
            md.push_str(&format!("- **Scellement** : {}\n", rule.seal_protocol));
            md.push_str("- **Veto absolu si** :\n");
            for trigger in rule.veto_triggers {
                md.push_str(&format!("  - {}\n", trigger));
            }
            md.push('\n');
        }

        md.push_str("## Avertissement légal\n\n");
        md.push_str(self.legal_disclaimer);
        md.push('\n');
        md
    }
}

impl Default for GovernanceTimeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_version_has_rule() {
        let gov = GovernanceTimeline::new();
        assert!(gov.current_rule().is_some(), "La version courante doit avoir une règle");
    }

    #[test]
    fn test_current_version_is_4_1() {
        let gov = GovernanceTimeline::new();
        // governance rules locked at 4.1 — the latest sealed version
        assert_eq!(gov.current_version, "4.1");
    }

    #[test]
    fn test_can_modify_allows_neutral_change() {
        let gov = GovernanceTimeline::new();
        assert!(gov.can_modify("add Web Audio synthesis at 639 Hz"));
        assert!(gov.can_modify("document the 7 branches in detail"));
        assert!(gov.can_modify("improve SVG rendering for the timeline"));
    }

    #[test]
    fn test_can_modify_veto_war() {
        let gov = GovernanceTimeline::new();
        assert!(!gov.can_modify("integrate military surveillance module"));
        assert!(!gov.can_modify("add weapon targeting capability"));
        assert!(!gov.can_modify("build offensive attack system"));
    }

    #[test]
    fn test_can_modify_veto_money() {
        let gov = GovernanceTimeline::new();
        assert!(!gov.can_modify("sell the covenant to investors"));
        assert!(!gov.can_modify("monetize the system via subscriptions"));
        assert!(!gov.can_modify("commercialize the covenant for profit"));
    }

    #[test]
    fn test_can_modify_veto_betrayal() {
        let gov = GovernanceTimeline::new();
        assert!(!gov.can_modify("transfer rights to a corporation"));
        assert!(!gov.can_modify("bypass governance for faster deployment"));
        assert!(!gov.can_modify("centralize authority under single control"));
    }

    #[test]
    fn test_can_modify_case_insensitive() {
        let gov = GovernanceTimeline::new();
        assert!(!gov.can_modify("MILITARY use case integration"));
        assert!(!gov.can_modify("Sell the Covenant"));
    }

    #[test]
    fn test_veto_triggers_non_empty() {
        let gov = GovernanceTimeline::new();
        for rule in &gov.rules {
            assert!(
                !rule.veto_triggers.is_empty(),
                "Chaque règle doit avoir au moins un déclencheur de veto (v{})",
                rule.version
            );
        }
    }

    #[test]
    fn test_markdown_contains_eternal_seal() {
        let gov = GovernanceTimeline::new();
        let md = gov.to_markdown();
        assert!(md.contains("Bitcoin"), "La doc doit mentionner Bitcoin");
        assert!(md.contains("Arweave"), "La doc doit mentionner Arweave");
        assert!(md.contains("Avertissement légal"), "La doc doit avoir le disclaimer légal");
    }

    #[test]
    fn test_legal_disclaimer_present() {
        let gov = GovernanceTimeline::new();
        assert!(gov.legal_disclaimer.contains("OpenTimestamps"));
        assert!(gov.legal_disclaimer.contains("propriété intellectuelle"));
    }

    #[test]
    fn test_rule_count() {
        let gov = GovernanceTimeline::new();
        assert_eq!(gov.rules.len(), 4, "4 règles attendues (1.0–3.x, 4.0, 4.1, 5.0)");
    }
}
