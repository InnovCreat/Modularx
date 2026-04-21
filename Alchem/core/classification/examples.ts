// Alchem — Vocabulaire exemplaire par catégorie
// Lookup table pour la conscience électronique. Grandit sans toucher la taxonomie.

export const IDENTITY_TERMS = Object.freeze({
  DEFINITION:  "Identity",
  PROPERTIES:  "Identity",
  SPECS:       "Identity",
  SYNONYMS:    "Identity",
  TAGS:        "Identity",
  SYMBOLS:     "Identity",
  SOLID:       "Identity",
} as const);

export const ACTION_TERMS = Object.freeze({
  TRIGGERS:    "Action",
  BEHAVIORS:   "Action",
  MECHANISMS:  "Action",
  FLUID:       "Action",
} as const);

export const CAPACITY_TERMS = Object.freeze({
  CAPACITIES:  "Capacity",
  CAN_PREFIX:  "Capacity",  // Convention: tout symbole Can* → Capacity
} as const);

export const SPACE_TERMS = Object.freeze({
  MODULES:     "Space",
  LAYERS:      "Space",
  MAIN:        "Space",
  INTERFACE:   "Space",
  FILTER:      "Space",
} as const);

export const CONTROL_TERMS = Object.freeze({
  INTERACTIONS: "Control",
  SECURITY:     "Control",
  ASSOCIATIONS: "Control",
  GUARDIANS:    "Control",
} as const);

export type IdentityTerm  = keyof typeof IDENTITY_TERMS;
export type ActionTerm    = keyof typeof ACTION_TERMS;
export type CapacityTerm  = keyof typeof CAPACITY_TERMS;
export type SpaceTerm     = keyof typeof SPACE_TERMS;
export type ControlTerm   = keyof typeof CONTROL_TERMS;
