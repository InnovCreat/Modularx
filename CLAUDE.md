# CLAUDE.md — Modularx / Luna / Alchem

## Identité du projet

**Modularx** est le workspace Rust/WASM de Isabel Sigouin.
**Alchem** est le framework structurel — loi architecturale, pas suggestion.
**Luna** est le système vivant qui tourne sur Alchem.

Branche de développement : `claude/explain-codebase-mm06b6gjkayowf1j-Mx5KR`

---

## Zones FROZEN — ne jamais modifier sans mot-clé explicite

Les fichiers suivants sont gelés. Ne les éditer que si le message contient
le mot-clé exact (en majuscules) : **`MODIFIE ALCHEM CORE`**

```
Alchem/core/rules/alchem.ts          ← lois architecturales
Alchem/core/ethics/covenant.ts       ← couche éthique (guide, pas loi)
Alchem/core/classification/          ← taxonomie 4D complète
Alchem/core/messages.ts              ← protocole message bus
Alchem/core/environment.ts           ← ENV racine
```

Les fichiers suivants sont **libres** — données, pas loi :
```
Alchem/templates/    ← building blocks, peuvent évoluer
Alchem/artifacts/    ← compositions, peuvent évoluer
Alchem/docs/         ← documentation vivante
src/                 ← code Rust, peut évoluer
```

---

## Règles permanentes

1. **ENV d'abord** — dans tout nouveau fichier Rust, les constantes d'environnement
   sont déclarées avant toute logique.

2. **Fixed Sacred Order** — le `tickOrder` d'un artifact ne change pas en cours
   d'exécution. Pour changer l'ordre, créer un nouvel artifact.

3. **Pas de magie** — aucune fonction ne doit appeler quelque chose qui n'existe pas.
   Tout ce qui est appelé doit être déclaré quelque part dans le projet.

4. **Bus seulement** — les modules ne s'appellent pas directement.
   Toute communication passe par le message bus.

5. **Templates = déclaratif** — un template ne contient pas de logique exécutable.
   Il contient des constantes, des types, des structures. Pas des fonctions qui
   appellent d'autres fonctions.

6. **Variables avant hardcode** — si une valeur peut varier d'un artifact à l'autre,
   elle doit être un `{{placeholder}}` dans le template, pas une valeur fixe.

---

## Classification 4D — rappel rapide

| Dimension   | Question          | Fichier    | Catégorie Alchem |
|-------------|-------------------|------------|------------------|
| DEFINITION  | WHAT IT IS        | `.types`   | Object Data      |
| BEHAVIOR    | WHAT IT DOES      | `.behavior`| Command Data     |
| CAPACITY    | CAN IT DO X?      | `Can*`     | Ability          |
| STRUCTURE   | WHERE IT LIVES    | directory  | Environment      |
| RELATION    | HOW IT CONNECTS   | `.guard`   | Security         |

---

## Mot-clés de rituel (système Luna)

`RÉVEIL` `SOMMEIL` `CHAOS` `HARMONIE` `ATTRACTION` `RÉPULSION` `AURORE` `RÉPARATION`

Toujours en majuscules. Toujours via le message bus. Jamais d'appel direct.

---

## Stack technique

- **Rust** + **WASM** via `wasm-bindgen`
- **Leptos 0.6** (CSR) — syntaxe `<For>` requiert `{}` autour des closures
- **Trunk** pour le build
- **serde / serde_json** pour la sérialisation des templates et artifacts
- **web-sys** avec features Audio + Canvas

---

## Fréquence de référence

**639 Hz** — fréquence de cohérence. Tout artifact en état `Resonant` opère à 639 Hz.
`COHERENCE_THRESHOLD = 0.7` — seuil minimum pour résonner.
`PHI = 1.618...` — ratio géométrique de la mémoire fractale (ChronoVision).
