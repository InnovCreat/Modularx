// ─────────────────────────────────────────────
// VERITAS HORTUS · gabarit · mod.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Canon
// Fonction    : Comprendre · Communiquer
// Gravité     : Forme
// Révolution  : interne
// Souveraineté: nulle
// Chaîne      : ─
// Statut      : graine (aucune implémentation encore)
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.4 — Deux Centres
// ─────────────────────────────────────────────
//
// Étape B de la séquence A→B→C→D d'auto-modulation.
// Miroir Rust de gabarits/FICHE_VIERGE.md.
//
// Aucune implémentation ici — c'est la forme typée à laquelle les
// modules se conformeront via le trait HasFiche (étape C).
//
// Tous les champs sont Option<...> parce qu'une case vide n'est pas
// un échec — c'est un signal.

/// Type d'état d'un module (§II du Langage — catégorie 07).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EtatType {
    Physique,
    Computationnel,
    Relationnel,
    Qualitatif,
}

/// En-tête — identité du module.
#[derive(Debug, Clone, Default)]
pub struct Entete {
    pub name_id:     Option<String>,
    pub categorie:   Option<String>,
    pub description: Option<String>,
}

/// Forme — qu'est-ce ? (sommet du triangle).
#[derive(Debug, Clone, Default)]
pub struct Forme {
    pub type_role:             Option<String>,
    pub proprietes_mesurables: Option<String>,
    pub etat_de_depart:        Option<String>,
    pub etats_possibles:       Option<String>,
    pub etat_type:             Option<EtatType>,
}

/// Lien — connecté à quoi ? (sommet du triangle).
#[derive(Debug, Clone, Default)]
pub struct Lien {
    pub dependances:    Option<String>,
    pub parent_child:   Option<String>,
    pub interlocuteurs: Option<String>,
}

/// Temps — comment ça change ? (sommet du triangle).
#[derive(Debug, Clone, Default)]
pub struct Temps {
    pub trigger:         Option<String>,
    pub elan:            Option<String>,
    pub cycle_frequence: Option<String>,
}

/// Chaîne d'activation — un déclenchement réel.
///
/// Forme(départ) → trigger → spark → action → action-communication
///              → concret → Forme(arrivée)
#[derive(Debug, Clone, Default)]
pub struct ChaineActivation {
    pub forme_depart:         Option<String>,
    pub trigger:              Option<String>,
    pub spark:                Option<String>,
    pub action:               Option<String>,
    pub action_communication: Option<String>,
    pub concret:              Option<String>,
    pub forme_arrivee:        Option<String>,
}

/// Cadre — règles, contexte, persistance.
#[derive(Debug, Clone, Default)]
pub struct Cadre {
    pub regles_locales:  Option<String>,
    pub regles_globales: Option<String>,
    pub contexte:        Option<String>,
    pub persistance:     Option<String>,
}

/// Signal — lecture méta d'une case vide.
///
/// Deux interprétations possibles : soit le module a un trou dans sa
/// conception, soit le langage n'a pas encore le mot pour ce cas.
#[derive(Debug, Clone, Default)]
pub struct Signal {
    pub case_manquante: Option<String>,
    pub lecture:        Option<String>,
}

/// Fiche de Module — miroir Rust de gabarits/FICHE_VIERGE.md.
///
/// Une fiche décrit ce qu'un module est, à quoi il est connecté, comment
/// il change, et comment il s'active. Tous les champs sont optionnels —
/// une case vide (`None`) n'est pas un échec, c'est un signal.
#[derive(Debug, Clone, Default)]
pub struct Fiche {
    pub entete:  Entete,
    pub forme:   Forme,
    pub lien:    Lien,
    pub temps:   Temps,
    pub chaine:  ChaineActivation,
    pub cadre:   Cadre,
    pub signaux: Vec<Signal>,
}

/// Contrat que remplit un module pour se présenter au système.
///
/// C'est le pivot de l'auto-modulation : chaque module qui implémente
/// ce trait expose sa fiche à l'exécution. Un futur GabaritRegistry
/// (étape D) pourra les collecter et piloter la recomposition.
pub trait HasFiche {
    fn fiche(&self) -> Fiche;
}
