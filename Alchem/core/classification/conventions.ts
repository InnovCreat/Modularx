// Alchem — Conventions de fichiers par couche
// Source unique de vérité pour les suffixes. Le suffixe déclare la couche.

export const FILE_CONVENTIONS = Object.freeze({

  // Définition Layer — données statiques, formes, specs (Object Data)
  PROPS:    ".props",
  TYPES:    ".types",

  // Behavioral Layer — messages et déclencheurs (Command Data)
  MSG:      ".msg",
  TRIG:     ".trig",

  // Behavioral Layer — logique fluide et mécanismes (Command Data)
  BEHAVIOR: ".behavior",
  MECH:     ".mech",

  // Relation Layer — sécurité, filtres, gardiens (Security)
  GUARD:    ".guard",

  // Structural Layer — implicite dans l'arborescence
  IMPLICIT: "(directory)",

} as const);

// Standard filenames inside every module (Alchem canonical form)
export const MODULE_FILES = Object.freeze({
  INTENT:    "module.md",   // intention déclarée — une phrase
  TYPES:     "types.ts",    // Object Data only
  MESSAGES:  "messages.ts", // Command Data & Messages
  CORE:      "core.ts",     // Main module logic
  INDEX:     "index.ts",    // Public interface — exports only
} as const);

export type ConventionKey    = keyof typeof FILE_CONVENTIONS;
export type ConventionSuffix = typeof FILE_CONVENTIONS[ConventionKey];
export type ModuleFileKey    = keyof typeof MODULE_FILES;
