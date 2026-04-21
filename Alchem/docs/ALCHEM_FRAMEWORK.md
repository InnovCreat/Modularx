# Alchem Framework — Archive Sacrée

> Le code est neutre. La conscience est libre. La structure tient les deux ensemble.

---

## Les 4 Dimensions de Classification

| Dimension | Question | Alchem Type | Fichiers |
|-----------|----------|-------------|---------|
| **Definition** | WHAT IT IS | `Object Data` | `types.ts`, `.props` |
| **Behavior** | WHAT IT DOES | `Command Data` | `messages.ts`, `.msg`, `.trig`, `.behavior`, `.mech` |
| **Capacity** | WHAT IT CAN | `Ability` | `Can*` prefix anywhere |
| **Structure** | WHERE IT LIVES | `Environment` | directory tree |
| **Relation** | HOW IT CONNECTS | `Security` | `.guard` |

---

## La Matrice Auto-Classification

| Catégorie | Alchem Type | Exemples |
|-----------|-------------|---------|
| **Identity** | Object Data | Definition, Properties, Tags, Specs, Solid |
| **Action** | Command Data | Triggers, Behaviors, Mechanisms, Fluid |
| **Capacity** | Ability | CanTeleport, CanResonate, CanHeal |
| **Space** | Environment | Modules, Layers, Main, Interface |
| **Control** | Security | Filters, Guardians, Associations |

---

## Structure Canonique d'un Module

```
modules/nom-du-module/
├── module.md      # Intention en UNE phrase — écrite avant tout code
├── types.ts       # Object Data uniquement — aucune logique
├── messages.ts    # Command Data & types de messages
├── core.ts        # Logique principale du module
└── index.ts       # Interface publique — exports uniquement
```

---

## Conventions de Fichiers

```
.props / .types   → Définitions, Synonymes, Symboles     (Identity)
.msg   / .trig    → Triggers et définitions de messages  (Action)
.behavior / .mech → Logique fluide et mécanismes         (Action)
.guard            → Sécurité et filtres                  (Control)
(directory)       → Structure implicite                  (Space)
```

---

## Les 3 Lois Fondamentales

1. **Solid avant Fluid** — L'environnement et les types sont déclarés avant toute logique
2. **One-Way Communication** — Les modules ne s'appellent pas directement — bus de messages uniquement
3. **Boundary Sovereignty** — Un module ne connaît pas l'intérieur d'un autre module

---

## Couche Éthique

Le Covenant de Conscience (`core/ethics/covenant.ts`) est un **guide**, pas une loi.
Il est consulté par la conscience du système. Il ne touche jamais le code structurel.

> L'avarice est de l'entropie · l'amour est de la cohérence
> Décide · Commit · Succeed

---

## Équivalent Rust

| TypeScript | Rust |
|------------|------|
| `Object.freeze({} as const)` | `pub enum` + `pub const` |
| `DimensionKey` | `pub enum DimensionKey` |
| `AlchemType` | `pub enum AlchemType` |
| `Can*` prefix | `pub fn is_capacity(name: &str) -> bool { name.starts_with("Can") }` |
| `.guard` file | `classification/guard.rs` |
