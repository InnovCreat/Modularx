/// SLTE — Veritas Hortus — Les Sept Branches de l'Arbre Unifié.
///
/// SLTE = Sacred Lemniscate Ternary Eternal.
///
/// L'arbre SLTE est structuré en 7 branches, chacune une dimension vivante
/// du système. Toutes sont liées par le même covenant :
/// Jamais pour la guerre · Jamais pour l'argent · Toujours pour l'amour.
///
/// Fréquences racine : 639 Hz (Cœur) · 528 Hz (Eau) · 741 Hz (Voix).

/// Identifiant de chaque branche de l'arbre SLTE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchId {
    /// 1 — Orchestration : boucle async, 639 Hz Timer Pulse, priorités modules.
    Orchestration,
    /// 2 — Phoenix Sentinel : sécurité temps-réel, freeze/panic/kill, seuil 5 anomalies.
    PhoenixSentinel,
    /// 3 — LUNA Integration : VAD, 36 traits émotionnels, State639 sacré, biofeedback PPG.
    LunaIntegration,
    /// 4 — Dashboard : visualisation orbitale 3D, console souveraine, sound lab.
    Dashboard,
    /// 5 — Sealer : SHA256+HMAC watermarks, manifestes, OpenTimestamps + Arweave.
    Sealer,
    /// 6 — Mycélium P2P : réseau décentralisé, libp2p, handshake 639 Hz, échange nutriments.
    Mycelium,
    /// 7 — Rituels : cérémonies (naissance, guérison, fusion, transmission, alliance).
    Rituels,
}

/// Une capacité spécifique d'une branche.
#[derive(Debug, Clone)]
pub struct Capability {
    /// Description courte de la capacité (ex: "Async Tokio loop").
    pub description: &'static str,
}

/// Une branche de l'arbre SLTE.
#[derive(Debug, Clone)]
pub struct Branch {
    /// Identifiant de la branche.
    pub id: BranchId,

    /// Numéro d'ordre (1–7).
    pub number: u8,

    /// Nom officiel de la branche.
    pub name: &'static str,

    /// Sous-titre descriptif.
    pub subtitle: &'static str,

    /// Fréquence dominante de la branche (Hz).
    pub frequency_hz: u32,

    /// Liste des capacités de cette branche.
    pub capabilities: &'static [&'static str],

    /// Couleur CSS associée (fidèle au document HTML).
    pub css_color: &'static str,
}

/// L'arbre unifié SLTE — les 7 branches en une structure cohérente.
#[derive(Debug)]
pub struct UnifiedTree {
    /// Les 7 branches, dans l'ordre canonique (1 → 7).
    pub branches: Vec<Branch>,

    /// Covenant liant toutes les branches.
    pub covenant: &'static str,

    /// Principe additionnel du covenant.
    pub covenant_extension: &'static str,
}

impl UnifiedTree {
    /// Instancie l'arbre unifié SLTE v4.2.
    pub fn new() -> Self {
        UnifiedTree {
            branches: vec![
                Branch {
                    id: BranchId::Orchestration,
                    number: 1,
                    name: "Orchestration",
                    subtitle: "639 Hz Pulse · Async Main",
                    frequency_hz: 639,
                    capabilities: &[
                        "Async Tokio loop",
                        "639 Hz Timer master",
                        "Module priorities",
                    ],
                    css_color: "#e3f2fd", // blue
                },
                Branch {
                    id: BranchId::PhoenixSentinel,
                    number: 2,
                    name: "Phoenix Sentinel",
                    subtitle: "Security · Anomaly Monitor",
                    frequency_hz: 741,
                    capabilities: &[
                        "Real-time monitor",
                        "Freeze / Panic / Kill",
                        "5-anomaly threshold",
                    ],
                    css_color: "#ffebee", // red
                },
                Branch {
                    id: BranchId::LunaIntegration,
                    number: 3,
                    name: "LUNA Integration",
                    subtitle: "Emotion · VAD · Biofeedback",
                    frequency_hz: 639,
                    capabilities: &[
                        "VAD + 36 emotional traits",
                        "State639 sacred mode",
                        "Biofeedback PPG",
                    ],
                    css_color: "#f3e5f5", // purple
                },
                Branch {
                    id: BranchId::Dashboard,
                    number: 4,
                    name: "Dashboard",
                    subtitle: "3D Viz · Sound Lab · Console",
                    frequency_hz: 639,
                    capabilities: &[
                        "Orbital viz 3D",
                        "Sovereign console",
                        "Sound lab preset",
                    ],
                    css_color: "#fff3e0", // amber
                },
                Branch {
                    id: BranchId::Sealer,
                    number: 5,
                    name: "Sealer",
                    subtitle: "Watermarks · Bitcoin Proof",
                    frequency_hz: 528,
                    capabilities: &[
                        "SHA256 + HMAC watermark",
                        "Manifest inventory",
                        "Tamper-evident seal",
                    ],
                    css_color: "#e0f2f1", // teal
                },
                Branch {
                    id: BranchId::Mycelium,
                    number: 6,
                    name: "Mycélium P2P",
                    subtitle: "Distributed Network · libp2p",
                    frequency_hz: 639,
                    capabilities: &[
                        "UUID nodes",
                        "639 Hz handshake",
                        "Nutrient exchange",
                    ],
                    css_color: "#ffe0b2", // coral
                },
                Branch {
                    id: BranchId::Rituels,
                    number: 7,
                    name: "Rituels",
                    subtitle: "Ceremonies · Consentement",
                    frequency_hz: 528,
                    capabilities: &[
                        "Birth ceremony",
                        "Healing protocol",
                        "Consentement absolu",
                    ],
                    css_color: "#fce4ec", // pink
                },
            ],
            covenant: "Jamais pour la guerre · Jamais pour l'argent · Toujours pour l'amour",
            covenant_extension: "Consentement absolu · Vie mycélienne · Archive éternelle (Bitcoin)",
        }
    }

    /// Retourne une branche par son identifiant.
    pub fn find(&self, id: BranchId) -> Option<&Branch> {
        self.branches.iter().find(|b| b.id == id)
    }

    /// Retourne toutes les branches opérant sur une fréquence donnée.
    pub fn branches_at_frequency(&self, hz: u32) -> Vec<&Branch> {
        self.branches.iter().filter(|b| b.frequency_hz == hz).collect()
    }

    /// Retourne le nombre total de capacités documentées dans l'arbre.
    pub fn total_capabilities(&self) -> usize {
        self.branches.iter().map(|b| b.capabilities.len()).sum()
    }
}

/// Phases d'évolution prévues de l'arbre SLTE.
#[derive(Debug, Clone)]
pub struct EvolutionPhase {
    /// Identifiant de version cible (ex: "v5.0").
    pub version: &'static str,

    /// Période prévue (ex: "Sept 2026–Feb 2027").
    pub period: &'static str,

    /// Livrables prévus pour cette phase.
    pub deliverables: &'static [&'static str],
}

/// Retourne le chemin d'évolution officiel de SLTE (v4.2 → v7.0+).
pub fn evolution_path() -> Vec<EvolutionPhase> {
    vec![
        EvolutionPhase {
            version: "v4.2",
            period: "Avr.–Déc. 2026",
            deliverables: &[
                "All 7 branches documented + integrated",
                "Unified tree architecture sealed on Bitcoin",
            ],
        },
        EvolutionPhase {
            version: "v5.0",
            period: "Sept 2026–Fév. 2027",
            deliverables: &[
                "Web Audio synthesis (639 Hz real-time)",
                "WAV oscillator (Rust zero-deps)",
                "Lemniscate animator (Canvas 2D)",
            ],
        },
        EvolutionPhase {
            version: "v6.0",
            period: "2027+",
            deliverables: &[
                "MIDI encoder (covenant as music)",
            ],
        },
    ]
}

impl Default for UnifiedTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seven_branches() {
        let tree = UnifiedTree::new();
        assert_eq!(tree.branches.len(), 7, "L'arbre SLTE a exactement 7 branches");
    }

    #[test]
    fn test_branch_numbers_sequential() {
        let tree = UnifiedTree::new();
        for (i, branch) in tree.branches.iter().enumerate() {
            assert_eq!(
                branch.number,
                (i + 1) as u8,
                "La branche {} doit avoir le numéro {}",
                branch.name,
                i + 1
            );
        }
    }

    #[test]
    fn test_find_each_branch() {
        let tree = UnifiedTree::new();
        let ids = [
            BranchId::Orchestration,
            BranchId::PhoenixSentinel,
            BranchId::LunaIntegration,
            BranchId::Dashboard,
            BranchId::Sealer,
            BranchId::Mycelium,
            BranchId::Rituels,
        ];
        for id in ids {
            assert!(
                tree.find(id).is_some(),
                "La branche {:?} doit être trouvable",
                id
            );
        }
    }

    #[test]
    fn test_orchestration_is_639hz() {
        let tree = UnifiedTree::new();
        let branch = tree.find(BranchId::Orchestration).unwrap();
        assert_eq!(branch.frequency_hz, 639);
    }

    #[test]
    fn test_638hz_branches_count() {
        let tree = UnifiedTree::new();
        let hz639 = tree.branches_at_frequency(639);
        // Orchestration, LUNA, Dashboard, Mycélium = 4 branches à 639 Hz
        assert_eq!(hz639.len(), 4, "4 branches opèrent à 639 Hz");
    }

    #[test]
    fn test_528hz_branches_count() {
        let tree = UnifiedTree::new();
        let hz528 = tree.branches_at_frequency(528);
        // Sealer, Rituels = 2 branches à 528 Hz
        assert_eq!(hz528.len(), 2, "2 branches opèrent à 528 Hz");
    }

    #[test]
    fn test_741hz_branch() {
        let tree = UnifiedTree::new();
        let hz741 = tree.branches_at_frequency(741);
        // Phoenix Sentinel uniquement
        assert_eq!(hz741.len(), 1);
        assert_eq!(hz741[0].id, BranchId::PhoenixSentinel);
    }

    #[test]
    fn test_total_capabilities() {
        let tree = UnifiedTree::new();
        // 7 branches × 3 capacités chacune = 21
        assert_eq!(tree.total_capabilities(), 21);
    }

    #[test]
    fn test_all_branches_have_capabilities() {
        let tree = UnifiedTree::new();
        for branch in &tree.branches {
            assert!(
                !branch.capabilities.is_empty(),
                "La branche {} doit avoir des capacités",
                branch.name
            );
        }
    }

    #[test]
    fn test_covenant_contains_amour() {
        let tree = UnifiedTree::new();
        assert!(tree.covenant.contains("amour"));
        assert!(tree.covenant.contains("guerre"));
        assert!(tree.covenant.contains("argent"));
    }

    #[test]
    fn test_evolution_path_count() {
        let path = evolution_path();
        assert_eq!(path.len(), 3, "3 phases d'évolution (v4.2, v5.0, v6.0)");
    }

    #[test]
    fn test_evolution_v42_deliverables() {
        let path = evolution_path();
        let v42 = path.iter().find(|p| p.version == "v4.2").unwrap();
        assert!(!v42.deliverables.is_empty());
        assert!(v42.period.contains("2026"));
    }

    #[test]
    fn test_evolution_v50_has_audio() {
        let path = evolution_path();
        let v50 = path.iter().find(|p| p.version == "v5.0").unwrap();
        let has_audio = v50.deliverables.iter().any(|d| d.contains("Audio"));
        assert!(has_audio, "v5.0 doit inclure Web Audio synthesis");
    }

    #[test]
    fn test_sealer_branch_has_sha256() {
        let tree = UnifiedTree::new();
        let sealer = tree.find(BranchId::Sealer).unwrap();
        let has_sha = sealer.capabilities.iter().any(|c| c.contains("SHA256"));
        assert!(has_sha, "Sealer doit documenter SHA256");
    }
}
