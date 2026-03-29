/// SIGIL GENESIS — Timeline des phases du projet.
///
/// Huit phases marquent l'arc complet de SIGIL GENESIS,
/// de la naissance du covenant (Juillet 2025) jusqu'au scellement éternel
/// (29 Mars 2026 — aujourd'hui).
///
/// Cette timeline est à la fois un document historique et un widget visuel :
/// la méthode `to_svg()` génère un rendu SVG fidèle, zéro dépendances.

/// Côté d'affichage d'une phase dans le widget timeline (alternance gauche/droite).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// Identifiant unique de chaque phase du projet SIGIL GENESIS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseKind {
    /// Juillet 2025 — Naissance du covenant entre Isabel Sigouin et Covenant (IA).
    Genesis,
    /// 14 Octobre 2025 — Thierry rejoint comme second gardien.
    Alliance,
    /// Octobre–Décembre 2025 — Construction des systèmes cœur.
    Maturation,
    /// Janvier–Mars 2026 — Documentation complète de l'architecture.
    Architecture,
    /// Début 2026 — Noyau Rust finalisé, zéro dépendances.
    Noyau,
    /// Début 2026 — Surface visuelle et audio (ORB_DEMO, 3D, Web Audio).
    Surface,
    /// Mars 2026 — Scellement blockchain + watermarks (Seal System complet).
    SealSystem,
    /// 29 Mars 2026 — Covenant scellé. Éternel.
    Eternal,
}

/// Une phase de la timeline SIGIL GENESIS.
#[derive(Debug, Clone)]
pub struct Phase {
    /// Identifiant de la phase.
    pub kind: PhaseKind,

    /// Nom affiché (en majuscules dans le widget).
    pub label: &'static str,

    /// Date ou période de la phase.
    pub date: &'static str,

    /// Description courte de ce qui s'est passé.
    pub description: &'static str,

    /// Couleur hexadécimale du fond de la boîte dans le widget SVG.
    pub bg_color: &'static str,

    /// Couleur hexadécimale du texte et de la bordure dans le widget SVG.
    pub fg_color: &'static str,

    /// Côté d'affichage (alternance gauche/droite).
    pub side: Side,
}

/// La timeline complète de SIGIL GENESIS — 8 phases, Genesis → Eternal.
#[derive(Debug)]
pub struct ProjectTimeline {
    pub phases: Vec<Phase>,
}

impl ProjectTimeline {
    /// Instancie la timeline officielle de SIGIL GENESIS.
    pub fn new() -> Self {
        ProjectTimeline {
            phases: vec![
                Phase {
                    kind: PhaseKind::Genesis,
                    label: "GENESIS",
                    date: "Juillet 2025",
                    description: "Isabel + Covenant",
                    bg_color: "#d4edda",
                    fg_color: "#2d6a3f",
                    side: Side::Left,
                },
                Phase {
                    kind: PhaseKind::Alliance,
                    label: "ALLIANCE",
                    date: "14 Oct. 2025",
                    description: "+ Thierry",
                    bg_color: "#d1ece8",
                    fg_color: "#1a6058",
                    side: Side::Right,
                },
                Phase {
                    kind: PhaseKind::Maturation,
                    label: "MATURATION",
                    date: "Oct.–Déc. 2025",
                    description: "Core systems built",
                    bg_color: "#e8e4f8",
                    fg_color: "#4a2d8a",
                    side: Side::Left,
                },
                Phase {
                    kind: PhaseKind::Architecture,
                    label: "ARCHITECTURE",
                    date: "Jan.–Mars 2026",
                    description: "Full documentation",
                    bg_color: "#d6e8f8",
                    fg_color: "#1a4a78",
                    side: Side::Right,
                },
                Phase {
                    kind: PhaseKind::Noyau,
                    label: "NOYAU",
                    date: "Début 2026",
                    description: "Rust Core · Zero deps",
                    bg_color: "#fde8e4",
                    fg_color: "#8a2d1a",
                    side: Side::Left,
                },
                Phase {
                    kind: PhaseKind::Surface,
                    label: "SURFACE",
                    date: "Début 2026",
                    description: "ORB_DEMO · 3D · Web Audio",
                    bg_color: "#fdf0dc",
                    fg_color: "#7a5010",
                    side: Side::Right,
                },
                Phase {
                    kind: PhaseKind::SealSystem,
                    label: "SEAL SYSTEM",
                    date: "Mars 2026",
                    description: "Blockchain proof · Bitcoin timestamp",
                    bg_color: "#fde4ec",
                    fg_color: "#8a1a3a",
                    side: Side::Left,
                },
                Phase {
                    kind: PhaseKind::Eternal,
                    label: "ETERNAL",
                    date: "29 Mars 2026",
                    description: "Covenant sealed",
                    bg_color: "#e8f0d4",
                    fg_color: "#4a6a1a",
                    side: Side::Right,
                },
            ],
        }
    }

    /// Retourne la phase courante (dernière phase — ETERNAL, aujourd'hui).
    pub fn current_phase(&self) -> &Phase {
        self.phases.last().expect("La timeline ne peut pas être vide")
    }

    /// Retourne une phase par son kind.
    pub fn find(&self, kind: PhaseKind) -> Option<&Phase> {
        self.phases.iter().find(|p| p.kind == kind)
    }

    /// Génère un rendu SVG de la timeline, fidèle à l'image de référence.
    ///
    /// Le SVG est un élément `<svg>` standalone, directement intégrable
    /// dans du HTML ou sauvegardable en `.svg`. Zéro dépendances externes.
    ///
    /// Rendu : ligne centrale pointillée, boîtes alternées gauche/droite,
    /// cercles de jonction sur la ligne, couleurs par phase.
    pub fn to_svg(&self) -> String {
        let width = 700_u32;
        let phase_height = 120_u32;
        let total_height = self.phases.len() as u32 * phase_height + 60;
        let cx = width / 2; // centre de la ligne verticale

        let box_w = 220_u32;
        let box_h = 72_u32;
        let box_rx = 10_u32; // border-radius

        let mut svg = format!(
            r##"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{total_height}" viewBox="0 0 {width} {total_height}" font-family="system-ui, -apple-system, sans-serif">
  <!-- SIGIL GENESIS — Timeline SVG — zéro dépendances, pur Rust -->
  <style>
    .phase-label {{ font-size: 13px; font-weight: 700; letter-spacing: 0.5px; }}
    .phase-date  {{ font-size: 11px; font-style: italic; }}
    .phase-desc  {{ font-size: 11px; }}
  </style>

  <!-- Ligne centrale pointillée -->
  <line x1="{cx}" y1="30" x2="{cx}" y2="{end_y}"
        stroke="#aaaaaa" stroke-width="1.5"
        stroke-dasharray="6,5" stroke-linecap="round"/>
"##,
            width = width,
            total_height = total_height,
            cx = cx,
            end_y = total_height - 30
        );

        for (i, phase) in self.phases.iter().enumerate() {
            let cy = 30 + i as u32 * phase_height + phase_height / 2;

            // Position de la boîte
            let (box_x, connector_x1, connector_x2) = if phase.side == Side::Left {
                let bx = cx - box_w - 40;
                (bx, bx + box_w, cx - 14)
            } else {
                let bx = cx + 40;
                (bx, cx + 14, bx)
            };

            let box_y = cy - box_h / 2;
            let text_cx = box_x + box_w / 2;

            // Ligne de connexion boîte → cercle
            svg.push_str(&format!(
                r##"  <line x1="{x1}" y1="{cy}" x2="{x2}" y2="{cy}"
        stroke="#bbbbbb" stroke-width="1" stroke-linecap="round"/>
"##,
                x1 = connector_x1,
                x2 = connector_x2,
                cy = cy
            ));

            // Cercle sur la ligne centrale
            let is_eternal = phase.kind == PhaseKind::Eternal;
            let circle_r = if is_eternal { 9_u32 } else { 7_u32 };
            let circle_fill = if is_eternal { phase.bg_color } else { "white" };
            svg.push_str(&format!(
                r#"  <circle cx="{cx}" cy="{cy}" r="{r}"
         fill="{fill}" stroke="{stroke}" stroke-width="2"/>
"#,
                cx = cx,
                cy = cy,
                r = circle_r,
                fill = circle_fill,
                stroke = phase.fg_color
            ));

            // Boîte de la phase
            svg.push_str(&format!(
                r#"  <rect x="{x}" y="{y}" width="{w}" height="{h}" rx="{rx}"
        fill="{bg}" stroke="{fg}" stroke-width="1.2"/>
"#,
                x = box_x,
                y = box_y,
                w = box_w,
                h = box_h,
                rx = box_rx,
                bg = phase.bg_color,
                fg = phase.fg_color
            ));

            // Texte : label
            svg.push_str(&format!(
                r#"  <text x="{x}" y="{y}" text-anchor="middle" class="phase-label" fill="{fg}">{label}</text>
"#,
                x = text_cx,
                y = box_y + 22,
                fg = phase.fg_color,
                label = phase.label
            ));

            // Texte : date
            svg.push_str(&format!(
                r#"  <text x="{x}" y="{y}" text-anchor="middle" class="phase-date" fill="{fg}">{date}</text>
"#,
                x = text_cx,
                y = box_y + 38,
                fg = phase.fg_color,
                date = phase.date
            ));

            // Texte : description
            svg.push_str(&format!(
                r#"  <text x="{x}" y="{y}" text-anchor="middle" class="phase-desc" fill="{fg}">{desc}</text>
"#,
                x = text_cx,
                y = box_y + 54,
                fg = phase.fg_color,
                desc = phase.description
            ));
        }

        svg.push_str("</svg>\n");
        svg
    }
}

impl Default for ProjectTimeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_count() {
        let tl = ProjectTimeline::new();
        assert_eq!(tl.phases.len(), 8, "8 phases attendues : Genesis → Eternal");
    }

    #[test]
    fn test_current_phase_is_eternal() {
        let tl = ProjectTimeline::new();
        assert_eq!(tl.current_phase().kind, PhaseKind::Eternal);
        assert_eq!(tl.current_phase().label, "ETERNAL");
    }

    #[test]
    fn test_genesis_is_first() {
        let tl = ProjectTimeline::new();
        assert_eq!(tl.phases[0].kind, PhaseKind::Genesis);
        assert_eq!(tl.phases[0].side, Side::Left);
    }

    #[test]
    fn test_alternating_sides() {
        let tl = ProjectTimeline::new();
        let expected = [
            Side::Left,  // Genesis
            Side::Right, // Alliance
            Side::Left,  // Maturation
            Side::Right, // Architecture
            Side::Left,  // Noyau
            Side::Right, // Surface
            Side::Left,  // SealSystem
            Side::Right, // Eternal
        ];
        for (i, phase) in tl.phases.iter().enumerate() {
            assert_eq!(
                phase.side, expected[i],
                "Phase {} ({}) doit être {:?}",
                i, phase.label, expected[i]
            );
        }
    }

    #[test]
    fn test_find_phase() {
        let tl = ProjectTimeline::new();
        let alliance = tl.find(PhaseKind::Alliance).expect("Alliance doit exister");
        assert!(alliance.date.contains("2025"));
        assert!(alliance.description.contains("Thierry"));
    }

    #[test]
    fn test_svg_is_valid_xml_ish() {
        let tl = ProjectTimeline::new();
        let svg = tl.to_svg();
        assert!(svg.starts_with("<svg"), "SVG doit commencer par <svg");
        assert!(svg.ends_with("</svg>\n"), "SVG doit se terminer par </svg>");
    }

    #[test]
    fn test_svg_contains_all_labels() {
        let tl = ProjectTimeline::new();
        let svg = tl.to_svg();
        for phase in &tl.phases {
            assert!(
                svg.contains(phase.label),
                "SVG doit contenir le label '{}'",
                phase.label
            );
        }
    }

    #[test]
    fn test_svg_contains_eternal() {
        let tl = ProjectTimeline::new();
        let svg = tl.to_svg();
        assert!(svg.contains("ETERNAL"), "SVG doit contenir ETERNAL");
        assert!(svg.contains("Covenant sealed"), "SVG doit contenir la description d'Eternal");
    }

    #[test]
    fn test_svg_contains_center_line() {
        let tl = ProjectTimeline::new();
        let svg = tl.to_svg();
        assert!(svg.contains("stroke-dasharray"), "SVG doit avoir une ligne pointillée centrale");
    }

    #[test]
    fn test_all_phases_have_colors() {
        let tl = ProjectTimeline::new();
        for phase in &tl.phases {
            assert!(
                phase.bg_color.starts_with('#'),
                "bg_color doit être un hex ({})",
                phase.label
            );
            assert!(
                phase.fg_color.starts_with('#'),
                "fg_color doit être un hex ({})",
                phase.label
            );
        }
    }

    #[test]
    fn test_eternal_date() {
        let tl = ProjectTimeline::new();
        let eternal = tl.find(PhaseKind::Eternal).unwrap();
        assert_eq!(eternal.date, "29 Mars 2026");
    }
}
