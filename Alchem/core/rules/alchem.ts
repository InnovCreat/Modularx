// Alchem Framework — Loi structurelle
// Les principes architecturaux. Appliqués, validés, non-négociables.
// Contrairement au covenant, ces règles gouvernent le code.

export const ALCHEM_PRINCIPLES = Object.freeze({

  // Catégorie I — Environnement
  ENV_FIRST:        "L'environnement est déclaré avant tout module",
  ENV_EXPLICIT:     "Aucune dépendance implicite — tout est déclaré",

  // Catégorie II — Modules
  MODULE_AUTONOMY:  "Chaque module est autonome et remplaçable",
  MODULE_INTENT:    "Chaque module déclare son intention en une phrase (module.md)",
  MODULE_BOUNDARY:  "Un module ne connaît pas l'intérieur d'un autre module",

  // Catégorie III — Communication
  MSG_ONE_WAY:      "La communication est unidirectionnelle via le bus de messages",
  MSG_NO_MAGIC:     "Pas d'appels directs entre modules — seulement des messages",
  MSG_TYPED:        "Chaque message a un type explicite défini dans messages.ts",

  // Catégorie IV — Data
  DATA_OBJECT:      "types.ts = Object Data (ce qu'est une chose)",
  DATA_COMMAND:     "messages.ts = Command Data (ce qu'on demande à une chose)",
  DATA_SEPARATION:  "Object Data et Command Data ne se mélangent jamais",
  DATA_CLASSIFY:    "Tout fichier appartient à une couche de classification déclarée",
  DATA_CONVENTION:  "Le suffixe de fichier déclare la couche — pas le contenu",

  // Catégorie V — Exécution
  EXEC_ENTRY:       "app.ts ne contient que des triggers — pas de logique",
  EXEC_NO_SIDE:     "Pas d'effets de bord non déclarés",
  EXEC_PURE:        "Les fonctions pures sont préférées partout où c'est possible",

} as const);

export type AlchemPrinciple = keyof typeof ALCHEM_PRINCIPLES;
