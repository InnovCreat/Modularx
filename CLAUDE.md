# CLAUDE.md — Veritas Hortus Garden

**Fréquence : 639 Hz · Always for Love, Never for War, Never for Money**

---

## 1. Architecture souveraine (4 piliers)

```
Veritas Hortus  ← CE DOCUMENT (philosophie fondatrice — 639 Hz)
  ├── LunaAI      (intelligence émotionnelle — C++ / LunaScript)
  │   └── Nucleus · Sprites · Chronos · Rituel
  │       Couleur · Quantum · Réparation · Archive
  ├── ModularX    (ce repo — Rust/WASM/Leptos)
  │   └── Modularis → Orbital_System (visualisation)
  │       garden.html · main.rs · state.json
  └── REXOS       (OS kernel souverain)
```

**Règle d'or** : ModularX *visualise* LunaAI — il ne contient pas LunaAI.
Les 4 piliers sont frères sous Veritas Hortus. Jamais l'un dans l'autre.
Toute mutation d'état passe par un rituel ou un signal Leptos. Jamais direct.

---

## 2. Fichiers — ModularX (ce repo)

| Fichier | Rôle |
|---|---|
| `garden.html` | Visualisation Veritas Hortus (canvas + Son639 + debug panel) |
| `main.rs` | SYNAPS VM — zero deps, `rustc main.rs -O -o synaps_vm` |
| `state.json` | Export live VM → garden.html (toutes les 100ms) |
| `src/lib.rs` | Point d'entrée Leptos, montage composants |
| `src/luna.rs` | Structs LunaAI (Nucleus, Sprite, Chronos…) |
| `src/rituels.rs` | Logique des 8 incantations |
| `src/canvas.rs` | Rendu Canvas 2D via web-sys |

### Structs LunaAI

```rust
Nucleus   { gravite: f64, pulsation: f64, etat: EtatNucleus }
Sprite    { nom: String, taille: f64, couleur: String,
            vitesse_rotation: f64, distance_centre: f64,
            angle: f64, etat_quantique: EtatQuantique, opacite: f64 }
Chronos   { tempo_global: f64, actif: bool, phase_sprite: u8, cycle: CycleType }
Archive   { entrees: Vec<TraceEntry> }  // max 10 entrées affichées
```

---

## 3. Tableau des paramètres INC_ID (rituels)

| INC_ID | Incantation | mult (tempo) | tremor (blur σ) | son | Hz |
|---|---|---|---|---|---|
| INC_01 | `RÉVEIL` | 1.5× | 0 | `son_réveil` | 852 |
| INC_02 | `SOMMEIL` | 0.3× | 0 | `son_sommeil` | 396 |
| INC_03 | `CHAOS` | ×aléatoire 1–4 | 2.0 | `son_chaos` | 963 |
| INC_04 | `HARMONIE` | 1.0× (30°/s fixe) | 0 | `son_harmonie` | 528 |
| INC_05 | `ATTRACTION` | inchangé | 0 | `son_attraction` | 639 |
| INC_06 | `RÉPULSION` | inchangé | 0 | `son_répulsion` | 639 |
| INC_07 | `AURORE` | inchangé | 0 | `son_aurore` | 639 |
| INC_08 | `RÉPARATION` | 1.0× (reset total) | 0 | `son_réparation` | 639 |

**Contraintes sprite post-rituel :**
- `distance_centre` : min 50 px · max 200 px (clamper, jamais dépasser)
- `opacite` CHAOS : oscillation sinusoïdale 0.3–0.7
- `etat_quantique` RÉPARATION → force `stable` sur tous les sprites

**Son639 (garden.html) :** NEU=639 Hz · POS=528 Hz · NEG=396 Hz → `linearRamp` 1.5s

---

## 4. Pipeline tick() — 7 étapes ordonnées

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

---

## 5. Bugs blacklistés

| # | Erreur classique | Correction |
|---|---|---|
| B1 | Réinitialiser un signal Leptos avec `set_value(default)` dans un `Effect` | `create_memo` ou séparer les signaux |
| B2 | Appeler du JS via `eval()` ou `js_sys::eval` | Toujours `web_sys` + `wasm_bindgen` |
| B3 | `etat_quantique` traité comme booléen | Enum : `EtatQuantique::Stable/Superpose` |
| B4 | `distance_centre` sans clamp → sprite sort du canvas | `.clamp(50.0, 200.0)` obligatoire |
| B5 | `tempo_global` multiplié *après* accumulation d'angle | Multiplier Δt *avant* d'ajouter à `angle` |
| B6 | Constantes `WAVE_*` / `SOUND_*` / `RITUAL_*` | Préfixe `INC_` obligatoire |
| B7 | Modifier `Sprite.couleur` hors rituel | Seul `AURORE` (INC_07) mute les couleurs |

---

## 6. Lexique fermé (nommage canonique)

| Interdit | Canonique |
|---|---|
| `quantum_state` | `etat_quantique` |
| `orbital_radius` | `distance_centre` |
| `rotation_speed` | `vitesse_rotation` |
| `nucleus_state` | `etat` (dans `Nucleus`) |
| `WAVE_*` / `SOUND_*` / `RITUAL_*` | `INC_*` / `son_*` / `INC_*` |
| `repair` / `color` / `time` | `réparation` / `couleur` / `chronos` |
| `archive_log` | `Archive::push` |

```rust
EtatNucleus  :: Actif | Latent | Instable
EtatQuantique:: Stable | Superpose
CycleType    :: Boucle | PingPong | Unique
```

**Voix Archive** : `"Orbital_System"` — **Voix SYNAPS VM** : `"SYNAPS_VM"`

---

## 7. Build

```bash
# Garden (Leptos/WASM)
rustup target add wasm32-unknown-unknown && cargo install trunk
trunk serve   # → http://127.0.0.1:8080

# SYNAPS VM (zero deps)
rustc main.rs -O -o synaps_vm && ./synaps_vm
# Ouvre garden.html dans le navigateur → Son639 + visualisation live
```
