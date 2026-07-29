# CLAUDE.md — SYNAPS VM / Veritas Hortus

## 1. Architecture

```
synaps_vm (single binary, zero deps)
  ├── VmCore          — stack machine, opcodes, program counter
  ├── FeedbackBuffer  — TP/FP/TN/FN counters + virtual vars 1000-1004
  ├── SynapsDispatcher— threshold adaptatif, profils, ternary state
  ├── TerminalRenderer— ANSI alternate screen, barres colorées
  ├── KeyboardWatcher — SPACE/ENTER/S/Q non-blocking
  └── JsonExport      — state.json toutes les 100ms → garden.html
```

**Règle d'or** : Zero dependencies. `rustc main.rs -O -o synaps_vm` — pas de Cargo.toml.

### Fichiers

| Fichier | Rôle |
|---|---|
| `main.rs` | Tout le code Rust (single-file, stdlib only) |
| `garden.html` | Visualisation web — fetch state.json toutes les 100ms |
| `state.json` | Export live généré par la VM |

---

## 2. Structs principales

```rust
VmCore       { stack: Vec<f64>, pc: usize, vars: HashMap<u32,f64>,
               state: VmState, frame: u64 }
FeedbackBuffer { tp: f64, fp: f64, tn: f64, fn_: f64 }
  // Virtual vars: 1000=Youden, 1001=Urgency, 1002=Sens, 1003=AUC, 1004=Spec
Dispatcher   { threshold: f64, profile: Profile, ternary: Ternary }
```

**États enum canoniques :**
```rust
VmState :: Running | Paused | Stasis | Halted
Ternary :: Neg | Neu | Pos          // Neg≤0.0, Neu≤0.5, Pos>0.5 (Youden)
Profile :: Conservative | Balanced | Aggressive
```

---

## 3. Opcodes

| Opcode | Effet |
|---|---|
| `LoadConst(f64)` | push valeur sur stack |
| `Add` | pop 2 → push somme |
| `Mul` | pop 2 → push produit |
| `Store(u32)` | pop → vars[id] |
| `Load(u32)` | vars[id] → push |
| `Jump(usize)` | pc = target |
| `JumpIf(usize)` | pop; si > 0 → pc = target |
| `FeedbackQuery(u32)` | virtual var 1000-1004 → push |
| `Broadcast` | log event + archive entry |
| `Halt` | VmState::Halted |

---

## 4. Métriques (FeedbackBuffer)

```
Sensitivity = TP / (TP + FN)
Specificity  = TN / (TN + FP)
Youden       = Sensitivity + Specificity - 1   // virtual var 1000
AUC          = (Sensitivity + Specificity) / 2  // virtual var 1003
Urgency      = 1.0 si Youden < 0.3 sinon 0.0   // virtual var 1001
```

**Profils Dispatcher :**

| Profil | threshold | sens_target | spec_target |
|---|---|---|---|
| Conservative | 0.7 | 0.6 | 0.9 |
| Balanced | 0.5 | 0.8 | 0.8 |
| Aggressive | 0.3 | 0.95 | 0.6 |

**Gradient descent :**
```rust
gradient  = (current_sens - current_spec) * 0.1
threshold = profile.t + gradient          // clamp 0.1..0.9
```

---

## 5. Pipeline tick() — 5 étapes

```
1. Keyboard poll (non-blocking) — SPACE/ENTER/S/Q
2. VmCore.step() — si Running ou (Paused && step_requested)
   a. fetch opcode[pc]
   b. execute → update stack/vars
   c. pc++
3. Feedback.update() — incrémenter TP/FP/TN/FN selon résultat
4. Dispatcher.adapt() — recalcul Youden → Ternary → gradient threshold
5. Render + JSON export (toutes les 100ms)
```

---

## 6. Bugs blacklistés

| # | Erreur | Correction |
|---|---|---|
| B1 | Stack underflow sans guard → panic | Toujours vérifier `stack.len() >= n` avant pop |
| B2 | Division par zéro dans Sensitivity/Specificity | Guard: `if (tp+fn_) == 0.0 { 0.0 } else { tp/(tp+fn_) }` |
| B3 | `threshold` hors [0.1, 0.9] | Toujours `.clamp(0.1, 0.9)` après gradient |
| B4 | Terminal corrompu si Ctrl-C sans cleanup | `defer` ou `Drop` impl pour restaurer terminal |
| B5 | state.json écrit partiellement (race) | Écrire dans un tmp puis `rename` atomique |
| B6 | `FeedbackQuery` sur var inconnue → panic | Retourner `0.0` pour tout id non reconnu |
| B7 | Boucle infinie sans sleep → 100% CPU | `thread::sleep(Duration::from_millis(16))` dans main loop |

---

## 7. Lexique

| Interdit | Canonique |
|---|---|
| `quantum_state` | `ternary` / `Ternary` |
| `true_positive` | `tp` (f64, pas usize) |
| `RITUAL_*` | `prog_*` (programmes) |
| `render_loop` | `tick()` |
| `threshold_value` | `threshold` |
| `log` / `print` | `archive_push()` |

**Philosophie** : `#![forbid(unsafe_code)]` implicite — zéro unsafe, zéro dépendances.
