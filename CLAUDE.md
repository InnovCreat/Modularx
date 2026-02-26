# CLAUDE.md — Orbital_System / Modularis

## 1. Architecture (3 axes)

```
Modularis (framework)
  └── Luna (living core — Rust structs)
      ├── Nucleus · Sprites · Chronos · Rituel
      ├── Couleur · Quantum · Réparation · Archive
      └── Interface_System
          └── Visual_System
              └── Orbital_System  ← CE REPO (Rust/WASM/Leptos)
```

**Règle d'or** : Orbital_System *visualise* Luna — il ne contient pas Luna.
Toute mutation de l'état passe par un rituel ou un signal Leptos. Jamais direct.

### Fichiers clés

| Fichier | Rôle |
|---|---|
| `src/lib.rs` | Point d'entrée Leptos, montage composants |
| `src/luna.rs` | Structs Luna (Nucleus, Sprite, Chronos…) |
| `src/rituels.rs` | Logique des 8 incantations |
| `src/canvas.rs` | Rendu Canvas 2D via web-sys |
| `src/archive.rs` | Log des événements système |
| `Cargo.toml` | Leptos 0.6, wasm-bindgen, web-sys, gloo-timers |
| `index.html` | Shell HTML minimal pour Trunk |

### Structs principales

```rust
Nucleus   { gravite: f64, pulsation: f64, etat: EtatNucleus }
Sprite    { nom: String, taille: f64, couleur: String,
            vitesse_rotation: f64, distance_centre: f64,
            angle: f64, etat_quantique: EtatQuantique, opacite: f64 }
Chronos   { tempo_global: f64, actif: bool, phase_sprite: u8, cycle: CycleType }
Archive   { entrees: Vec<TraceEntry> }  // max 10 entrées affichées
```

---

## 2. Tableau des paramètres INC_ID (rituels)

| INC_ID | Incantation | mult (tempo) | tremor (blur σ) | son |
|---|---|---|---|---|
| INC_01 | `RÉVEIL` | 1.5× | 0 | `son_réveil` |
| INC_02 | `SOMMEIL` | 0.3× | 0 | `son_sommeil` |
| INC_03 | `CHAOS` | ×aléatoire 1–4 | 2.0 | `son_chaos` |
| INC_04 | `HARMONIE` | 1.0× (30°/s fixe) | 0 | `son_harmonie` |
| INC_05 | `ATTRACTION` | inchangé | 0 | `son_attraction` |
| INC_06 | `RÉPULSION` | inchangé | 0 | `son_répulsion` |
| INC_07 | `AURORE` | inchangé | 0 | `son_aurore` |
| INC_08 | `RÉPARATION` | 1.0× (reset total) | 0 | `son_réparation` |

**Contraintes sprite post-rituel :**
- `distance_centre` : min 50 px · max 200 px (clamper, jamais dépasser)
- `opacite` CHAOS : oscillation sinusoïdale 0.3–0.7
- `etat_quantique` RÉPARATION → force `stable` sur tous les sprites

---

## 3. Pipeline tick() — 7 étapes ordonnées

```
1. Lire tempo_global (Chronos) — si !actif → return early, skip tout
2. Δangle = vitesse_rotation × tempo_global × Δt(s)  → angle += Δangle
3. Quantum check : si etat_quantique == superposé → calculer ghost_offset (sin wave)
4. Gravité application : distance_centre ajustée par gravite (attraction/répulsion)
5. Rendu Canvas :
     a. clearRect canvas
     b. dessiner orbites (paths pointillés, couleur sprite)
     c. dessiner Nucleus (cercle pulsant, couleur selon état)
     d. dessiner sprites (+ ghost si superposé, blur filter si tremor > 0)
6. Archive.push si événement rituel dans ce tick
7. request_animation_frame → prochain tick
```

**Ordre impératif** : gravité (étape 4) s'applique *avant* le rendu (étape 5),
tempo (étape 1) s'évalue *avant* le calcul d'angle (étape 2).

---

## 4. Bugs blacklistés (ne pas réintroduire)

| # | Erreur classique | Correction |
|---|---|---|
| B1 | Réinitialiser un signal Leptos avec `set_value(default)` dans un `Effect` | Utiliser `create_memo` ou séparer les signaux de reset |
| B2 | Appeler du JS via `eval()` ou `js_sys::eval` | Toujours passer par `web_sys` + `wasm_bindgen` |
| B3 | `etat_quantique` traité comme booléen (`true/false`) | C'est un enum : `EtatQuantique::Stable` / `EtatQuantique::Superpose` |
| B4 | `distance_centre` sans clamp → sprite sort du canvas | Toujours `distance_centre.clamp(50.0, 200.0)` |
| B5 | `tempo_global` multiplié *après* accumulation d'angle | Multiplier Δt par `tempo_global` *avant* d'ajouter à `angle` |
| B6 | Constantes nommées `WAVE_*` / `SOUND_*` / `RITUAL_*` | Voir Lexique §5 — préfixe `INC_` obligatoire |
| B7 | Modifier `Sprite.couleur` directement depuis le canvas | Seul le rituel `AURORE` (INC_07) est autorisé à muter les couleurs |

---

## 5. Lexique fermé (nommage canonique)

**Noms de modules — toujours en français :**

| Interdit | Canonique |
|---|---|
| `quantum_state` | `etat_quantique` |
| `orbital_radius` | `distance_centre` |
| `rotation_speed` | `vitesse_rotation` |
| `nucleus_state` | `etat` (dans `Nucleus`) |
| `WAVE_*` | `INC_*` (incantations) |
| `SOUND_*` | `son_*` (champs audio) |
| `RITUAL_*` | `INC_*` |
| `repair` | `réparation` |
| `color` | `couleur` |
| `time` / `clock` | `chronos` |
| `archive_log` | `Archive::push` |

**États enum canoniques :**

```rust
EtatNucleus  :: Actif | Latent | Instable
EtatQuantique:: Stable | Superpose          // pas "Superposé" (accent évité en Rust)
CycleType    :: Boucle | PingPong | Unique
```

**Voix Archive** : toujours `"Orbital_System"` (pas `"system"`, pas `"orbital"`)

---

## 6. Build & dev

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
trunk serve          # dev → http://127.0.0.1:8080
trunk build --release  # prod → dist/
```

**Stack** : Leptos 0.6 · wasm-bindgen · web-sys · gloo-timers · Zero JS
