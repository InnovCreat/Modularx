// Alchem — Matrice de classification (5 catégories → Alchem type)

export const CATEGORIES = Object.freeze({

  // Identity → ce qu'une chose EST   → .props / .types
  IDENTITY: "Object Data",

  // Action   → ce qu'une chose FAIT  → .msg / .trig / .behavior / .mech
  ACTION:   "Command Data",

  // Capacity → ce qu'une chose PEUT  → Can* prefix
  CAPACITY: "Ability",

  // Space    → où une chose VIT      → directory structure
  SPACE:    "Environment",

  // Control  → comment c'est FILTRÉ  → .guard
  CONTROL:  "Security",

} as const);

export type CategoryKey  = keyof typeof CATEGORIES;
export type CategoryType = typeof CATEGORIES[CategoryKey];
