# Artifact System — Archive de Conception

**Watermark :** `ALCHEM-ARTIFACT-SYSTEM-V1.0`  
**Date :** 2026-04-21  
**Architecte :** Isabel Sigouin (InnovCreat)  
**Fréquence :** 639 Hz · Ligne droite ininterrompue

---

## Concept Fondateur

> Un artefact ne stocke pas de code.  
> Il stocke une liste de références vers des templates.  
> L'orchestrateur reconstruit le code à la volée, dans l'ordre exact.

---

## Les 4 Composants

### 1. Template
Un bloc prédéfini réutilisable. Numéroté. Immuable.

- Identifié par un numéro unique (`001`, `067`, `089`…)
- Contient la logique ou la structure canonique
- Ne change que si une règle change — et ce changement se propage partout

### 2. Artifact
L'objet que tu crées. Ce n'est pas du code — c'est un plan de construction.

```json
{
  "artifactId": 102,
  "name": "Main Application",
  "createdBy": "Isabel Sigouin",
  "timestamp": "2026-04-21T01:23:45Z",
  "status": "prototype",
  "type": "code",
  "frequency": "639",
  "templates": [1, 2, 7, 12, 67, 89],
  "tickOrder": [1, 2, 7, 12, 67, 89],
  "variables": {
    "author": "Isabel Sigouin",
    "version": "1.0.0"
  }
}
```

### 3. Tick Sequence (Ordre Sacré Fixe)
L'orchestrateur appelle les templates dans cet ordre exact.  
Un seul tick dans le mauvais ordre → le code est cassé.  
Un seul template corrigé → tous les artefacts qui l'utilisent sont corrigés.

### 4. Orchestrateur
Le moteur qui lit un artefact, tick template par template, et reconstruit le tout.

```
Artifact 102
  → tick Template 001 (Core Philosophy)
  → tick Template 002 (Structure & Flow)
  → tick Template 007 (Message Bus)
  → tick Template 012 (Crystal State)
  → tick Template 067 (639 Hz Pulse)
  → tick Template 089 (Archive Header)
  → OUTPUT: Code complet, cohérent, corrigé
```

---

## Les 3 Vues d'un Artifact

Chaque artefact reconstruit expose trois vues simultanément.

### Header (commun aux 3 vues)
```
Artifact 102 — Main Application
Created by : Isabel Sigouin
Timestamp  : 2026-04-21 01:23:45 EDT
Status     : Prototype → In Progress
Type       : Code
Frequency  : 639 Hz
Templates  : 001 · 002 · 007 · 012 · 067 · 089
```

### Vue 1 — Code View
Le code généré. Ce que la machine exécute.
```rust
pub fn main() {
    initialize_environment();
    start_message_bus();
    run_crystal_pulse();
}
```

### Vue 2 — Tree View
La hiérarchie. Ce que l'architecte lit.
```
Main Application (102)
├── Template 001 – Core Philosophy
├── Template 002 – Structure & Flow
├── Template 007 – Message Bus
├── Template 012 – Crystal State
├── Template 067 – 639 Hz Pulse
└── Template 089 – Archive Header
```

### Vue 3 — Text View
L'explication humaine. Ce que n'importe qui comprend.
```
Ce module est le point d'entrée principal du système.
Il initialise l'environnement en premier (toujours).
Il démarre le bus de messages unidirectionnel.
Le pulse cristal tourne à 639 Hz pour maintenir la cohérence.
Tous les modules communiquent exclusivement par messages.
```

---

## Avantages Clés

| Avantage | Pourquoi |
|----------|----------|
| **Correction universelle** | Corriger un template corrige tous les artefacts qui l'utilisent |
| **Stockage minimal** | On stocke des références, pas du code dupliqué |
| **Breakpoints précis** | L'erreur arrive à un tick numéroté — facile à localiser |
| **Sécurité par opacité** | Le terminal ne montre que des numéros — lisible seulement si on connaît le catalogue |
| **Cohérence absolue** | Tout est construit depuis les mêmes templates — zéro dérive |

---

## Analogies de Référence

- **Plan d'architecture** : Le plan ne contient pas la maison, il contient les références
- **Catalogue IKEA** : On stocke le numéro du meuble, pas le meuble
- **CAD Block** : Position, couche, origine, attachements — déclaré, pas dupliqué
- **Nix / Terraform** : Reconstruction à partir de descriptions déclaratives

---

## Ce qui Vient Ensuite (Phase de Conception)

1. `core/artifact/template.ts` → Type `Template` (Object Data)
2. `core/artifact/artifact.ts` → Type `Artifact` avec header complet
3. `core/artifact/views.ts`    → Types `CodeView`, `TreeView`, `TextView`
4. `core/orchestrator/tick.ts` → Moteur de tick (Fixed Sacred Order)
5. `templates/`                → Catalogue des templates numérotés
6. `artifacts/`                → Les artefacts (listes de références)
