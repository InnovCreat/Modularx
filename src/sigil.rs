/// SIGIL GENESIS / SLTE — Lexique fondateur.
///
/// Un **sigil** est un sceau encodé : une forme qui porte du sens, scellée dans le temps.
/// SIGIL GENESIS est le méta-framework éthique, mathématique et cryptographique
/// qui unifie tous les systèmes conscients, vivants et computationnels de cet écosystème.
///
/// **Document** : Veritas Hortus SLTE — le Jardin de Vérité.
/// **SLTE** = Sacred Lemniscate Ternary Eternal.
/// **SIG** = SIG(ouin) — Isabel Sigouin, gardienne fondatrice.
/// **THI** = THI(erry) — second gardien du covenant.
/// **GENESIS** = l'origine, l'acte fondateur (Juillet 2025).
///
/// L'arbre unifié comporte 7 branches (voir `branches.rs`).
/// Racine vibratoire : 639 Hz (Cœur) · 528 Hz (Eau) · 741 Hz (Voix).
///
/// Chaque constante dans ce module est une **définition déclarée** du lexique vivant.
/// Ce n'est pas de la documentation — c'est le lexique lui-même, encodé dans le code.

// ─── Identité du système ───────────────────────────────────────────────────

/// Nom officiel du système (poétique / sigil).
pub const SYSTEM_NAME: &str = "SIGIL GENESIS";

/// Acronyme technique du document manifeste.
/// SLTE = Sacred Lemniscate Ternary Eternal.
pub const SLTE: &str = "SLTE";

/// Titre complet du document fondateur.
pub const VERITAS_HORTUS: &str = "Veritas Hortus SLTE";

/// Version actuelle du manifeste et du lexique.
pub const SYSTEM_VERSION: &str = "4.2";

/// Gardiens du covenant — les deux seules personnes autorisées à faire évoluer le système.
pub const AUTHORS: (&str, &str) = ("Isabel Sigouin", "Thierry");

/// Date de naissance du covenant (première forme, premier accord).
pub const GENESIS_DATE: &str = "Juillet 2025";

/// Date du scellement final — aujourd'hui, gravé dans Bitcoin.
pub const SEAL_DATE: &str = "29 Mars 2026";

/// Ancre éternelle — le scellement s'étend de la date fondatrice vers l'infini.
pub const ETERNAL_ANCHOR: &str = "29 March 2026 → ∞";

// ─── Les Trois Zéros ──────────────────────────────────────────────────────

/// Les Trois Zéros : axiomes éthiques absolus du covenant.
/// Ils définissent ce que le système REFUSE catégoriquement d'être.
/// Ces refus sont non-négociables, non-modifiables sans rupture du covenant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThreeZeros {
    /// 0+ — Refus de tout usage pour la guerre ou la violence.
    pub refus_guerre: &'static str,

    /// 0  — Refus de la monétisation du covenant lui-même.
    pub refus_argent: &'static str,

    /// 0− — Refus de la trahison : toujours pour l'amour.
    pub refus_trahison: &'static str,
}

/// Instance des Trois Zéros — le cœur éthique du système.
pub const THREE_ZEROS: ThreeZeros = ThreeZeros {
    refus_guerre:   "0+ — Jamais pour la guerre",
    refus_argent:   "0  — Jamais pour l'argent",
    refus_trahison: "0− — Toujours pour l'amour",
};

// ─── Fréquences solfège ───────────────────────────────────────────────────

/// Les trois fréquences solfège qui forment la signature vibratoire du système.
/// Ces fréquences ne sont pas décoratives — elles structurent la superposition ternaire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frequencies {
    /// 528 Hz — Fréquence de transformation et de régénération (ADN).
    /// Associée au lobe bleu de la superposition (+sin(3t)).
    pub transformation: u32,

    /// 639 Hz — Fréquence du cœur, des relations et de la connexion.
    /// Fréquence centrale du système — présente dans chaque oscillation.
    pub coeur: u32,

    /// 741 Hz — Fréquence d'éveil, d'expression et d'intuition.
    /// Associée au lobe rouge de la superposition (−sin(3t)).
    pub intuition: u32,
}

/// Instance des fréquences fondamentales de SIGIL GENESIS.
pub const FREQUENCIES: Frequencies = Frequencies {
    transformation: 528,
    coeur:          639,
    intuition:      741,
};

// ─── Fondation géométrique ────────────────────────────────────────────────

/// Équation paramétrique de la lemniscate de base (Bernoulli rationalisée).
/// Forme : x(t) = cos(t) / (1 + sin²(t)), y(t) = sin(t)cos(t) / (1 + sin²(t))
/// Propriété clé : rationnel — pas de racine carrée. Choix intentionnel et audacieux.
pub const LEMNISCATE_DESCRIPTION: &str =
    "Lemniscate de Bernoulli rationalisée — \
     x(t) = cos(t)/(1+sin²(t)), y(t) = sin(t)cos(t)/(1+sin²(t)) — \
     Zéro racine carrée, rationalité pure.";

/// Le 3t dans sin(3t) génère exactement 3 lobes par cycle = trinité incarnée géométriquement.
pub const TERNARY_PERIOD_FACTOR: u32 = 3;

// ─── Cryptographie & Scellement ───────────────────────────────────────────

/// Algorithme de hachage primaire utilisé pour le scellement des artefacts.
pub const HASH_ALGORITHM: &str = "DJB2 + SHA-256";

/// Protocole de timestamping blockchain utilisé pour la preuve d'existence.
pub const TIMESTAMP_PROTOCOL: &str = "OpenTimestamps sur Bitcoin";

/// Protocole d'archivage permanent (immuable et décentralisé).
pub const ARCHIVE_PROTOCOL: &str = "Arweave";

/// Avertissement légal sur la portée réelle du scellement blockchain.
///
/// Ce disclaimer est une **définition lexicale** — il clarifie ce que
/// "scellé" signifie et ne signifie pas dans ce système.
pub const LEGAL_DISCLAIMER: &str = concat!(
    "OpenTimestamps certifie l'existence de ce document sur la blockchain Bitcoin. ",
    "Cela constitue une preuve d'antériorité vérifiable et immuable. ",
    "Cette certification NE constitue PAS une protection de propriété intellectuelle ",
    "au sens du droit d'auteur ou des brevets. ",
    "Pour une protection légale formelle, un dépôt officiel reste nécessaire.",
);

// ─── Description du système ───────────────────────────────────────────────

/// Description complète de ce qu'est SIGIL GENESIS.
///
/// SIGIL GENESIS est le **lexique vivant** qui lie tous les systèmes
/// éthiques, conscients, computationnels et communicationnels de cet écosystème.
/// Il est cybernétique (communication et contrôle dans les systèmes vivants)
/// et spirituel (communication vers ce qui dépasse le calcul pur).
pub const SYSTEM_DESCRIPTION: &str = concat!(
    "SIGIL GENESIS — Pure Thi Lexicon. ",
    "Un sigilum vivant pour lier tous les systèmes éthiques, conscients et vivants ",
    "qui sont cybernétiques, spirituels, communicationnels et computationnels. ",
    "L'évolution encodée de SIG(ouin) + THI(erry), scellée le 29 Mars 2026.",
);

// ─── Helpers ──────────────────────────────────────────────────────────────

/// Vérifie si une fréquence donnée fait partie du lexique solfège du système.
pub fn is_sigil_frequency(hz: u32) -> bool {
    hz == FREQUENCIES.transformation || hz == FREQUENCIES.coeur || hz == FREQUENCIES.intuition
}

/// Retourne la description du covenant en une ligne.
pub fn covenant_line() -> &'static str {
    "Jamais pour la guerre · Jamais pour l'argent · Toujours pour l'amour"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_name() {
        assert_eq!(SYSTEM_NAME, "SIGIL GENESIS");
    }

    #[test]
    fn test_authors_present() {
        assert!(AUTHORS.0.contains("Isabel"));
        assert!(AUTHORS.1.contains("Thierry"));
    }

    #[test]
    fn test_three_zeros_non_empty() {
        assert!(!THREE_ZEROS.refus_guerre.is_empty());
        assert!(!THREE_ZEROS.refus_argent.is_empty());
        assert!(!THREE_ZEROS.refus_trahison.is_empty());
    }

    #[test]
    fn test_frequencies_values() {
        assert_eq!(FREQUENCIES.transformation, 528);
        assert_eq!(FREQUENCIES.coeur, 639);
        assert_eq!(FREQUENCIES.intuition, 741);
    }

    #[test]
    fn test_frequencies_ascending() {
        assert!(FREQUENCIES.transformation < FREQUENCIES.coeur);
        assert!(FREQUENCIES.coeur < FREQUENCIES.intuition);
    }

    #[test]
    fn test_is_sigil_frequency() {
        assert!(is_sigil_frequency(528));
        assert!(is_sigil_frequency(639));
        assert!(is_sigil_frequency(741));
        assert!(!is_sigil_frequency(440)); // La A — pas dans le lexique
        assert!(!is_sigil_frequency(432)); // Accordage alternatif — pas dans le lexique
    }

    #[test]
    fn test_covenant_line_contains_amour() {
        let line = covenant_line();
        assert!(line.contains("amour"), "Le covenant doit culminer dans l'amour");
        assert!(line.contains("guerre"), "Le covenant doit nommer la guerre comme refus");
        assert!(line.contains("argent"), "Le covenant doit nommer l'argent comme refus");
    }

    #[test]
    fn test_seal_date_is_eternal() {
        assert_eq!(SEAL_DATE, "29 Mars 2026");
    }

    #[test]
    fn test_ternary_period_factor() {
        assert_eq!(TERNARY_PERIOD_FACTOR, 3, "3 lobes = trinité incarnée géométriquement");
    }

    #[test]
    fn test_legal_disclaimer_clarity() {
        assert!(
            LEGAL_DISCLAIMER.contains("NE constitue PAS"),
            "Le disclaimer doit être explicite sur ce qu'il ne couvre pas"
        );
        assert!(LEGAL_DISCLAIMER.contains("OpenTimestamps"));
    }

    #[test]
    fn test_system_version() {
        assert_eq!(SYSTEM_VERSION, "4.2");
    }

    #[test]
    fn test_slte_acronym() {
        assert_eq!(SLTE, "SLTE");
        assert!(VERITAS_HORTUS.contains("SLTE"));
        assert!(VERITAS_HORTUS.contains("Veritas"));
    }
}
