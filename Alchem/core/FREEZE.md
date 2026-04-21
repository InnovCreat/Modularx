# FREEZE — Alchem Core

Ce répertoire est gelé.

Les fichiers ici sont des **lois**, pas du code évolutif.
Ils définissent les règles que tous les autres fichiers doivent respecter.
Les modifier, c'est changer les fondations pendant que la maison est debout.

## Pour modifier un fichier ici

Le message doit contenir le mot-clé exact : **`MODIFIE ALCHEM CORE`**

Sans ce mot-clé, aucune modification ne doit être appliquée, même si
la demande semble raisonnable.

## Ce qui est gelé et pourquoi

| Fichier | Pourquoi gelé |
|---------|---------------|
| `rules/alchem.ts` | 16 principes architecturaux — la loi du framework |
| `ethics/covenant.ts` | 14 guides moraux — la conscience, pas la structure |
| `classification/` | Taxonomie 4D complète — le vocabulaire du système |
| `messages.ts` | Protocole du bus — changer ça casse tous les modules |
| `environment.ts` | ENV racine — déclarée en premier, toujours |

## Ce qui peut évoluer sans restriction

- `Alchem/templates/` — les building blocks grandissent avec le projet
- `Alchem/artifacts/` — les compositions sont toujours provisoires
- `src/` — le code Rust s'adapte aux besoins

---

*Gelé par Isabel Sigouin. Alchem v1.0.0.*
