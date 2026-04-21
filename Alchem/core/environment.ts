// Alchem — Environnement root
// Déclaré en premier. Aucun module ne s'initialise avant ceci.

export const ENV = Object.freeze({
  SYSTEM:   "Alchem",
  VERSION:  "1.0.0",
  RUNTIME:  "browser" as "browser" | "server" | "wasm",
  DEBUG:    false,
} as const);

export type Runtime = typeof ENV.RUNTIME;
