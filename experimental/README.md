# experimental/

**Espace hors crate. Rien ici n'est compilé.**

Ce dossier accueille les brouillons, prototypes et pièces retirées du build principal mais qu'on ne veut ni supprimer ni laisser orpheliner à l'intérieur de `src/`.

## Doctrine

- Aucun fichier de `experimental/` n'est référencé par `Cargo.toml` ni par un `pub mod` dans `src/`.
- Chaque fichier conserve sa signature Constitution/Langage complète, augmentée d'un préfixe `EXPERIMENTAL` en tête.
- Un champ `Statut: experimental` dans la signature signale que la souveraineté et les invariants déclarés sont *aspirationnels* — ils décrivent ce que le module fera une fois réintégré, pas ce qu'il fait maintenant.
- La ré-intégration se fait explicitement : déplacer le fichier dans `src/<organe>/`, ajouter `pub mod ...;` dans le `mod.rs` de l'organe, faire compiler, mettre à jour le manifest.

## Pourquoi ce dossier existe

Trois raisons possibles pour qu'un fichier atterrisse ici :

1. **Draft** — le fichier existe mais l'organe ne l'a pas encore adopté (pattern actuel de `material_advanced.rs`).
2. **Retrait temporaire** — code fonctionnel retiré du build pour une raison ciblée (refactor en cours, dépendance à venir).
3. **Alternative** — variante d'un composant existant, gardée pour comparaison ou pour retour possible.

Aucun rapport avec l'idée d'un « bac à sable » libre : les invariants du Covenant s'appliquent ici comme partout ailleurs dans le repo.

## Contenu actuel

- `render/material_advanced.rs` — matériau avancé à 13 uniforms (fresnel/glow/quantum/crystalline/holographic/audio-reactivity). Utilise le shader `assets/shaders/sacred_pulse_advanced.wgsl`. Retiré du build parce que jamais déclaré dans `src/render/mod.rs`. Signalé dans `manifests/render.yaml` sous `gaps`.

## Ré-intégration

Quand un fichier de `experimental/` est prêt :

1. `git mv experimental/<organe>/<fichier>.rs src/<organe>/<fichier>.rs`
2. Ajouter `pub mod <fichier>;` dans `src/<organe>/mod.rs`
3. Retirer le bloc `EXPERIMENTAL` et le `Statut: experimental` de la signature
4. Mettre à jour `manifests/<organe>.yaml` :
   - Retirer la mention `gaps`
   - Ajouter le fichier à `components`
5. `cargo build` doit passer avant le commit
