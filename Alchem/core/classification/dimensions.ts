// Alchem — Couches de classification (WHAT / DOES / LIVES / CONNECTS)

export const DIMENSIONS = Object.freeze({

  // Couche I — Définition (ce qu'est une chose) → statique, descriptif
  DEFINITION: "Object Data",

  // Couche II — Comportement (ce que fait une chose) → logique, mouvement
  BEHAVIOR:   "Command Data",

  // Couche III — Capacité (ce qu'une chose peut faire) → potentiel déclaré
  CAPACITY:   "Ability",

  // Couche IV — Structure (où une chose vit) → hiérarchie, espace
  STRUCTURE:  "Environment",

  // Couche V — Relation (comment une chose se connecte) → flux, sécurité
  RELATION:   "Security",

} as const);

export type DimensionKey = keyof typeof DIMENSIONS;
export type AlchemType   = typeof DIMENSIONS[DimensionKey];
