// Template — Object Data uniquement. Le bloc prédéfini réutilisable.

import type { AlchemType } from "../classification/dimensions";

export interface Template {
  readonly id:       number;          // numéro unique — immuable
  readonly name:     string;          // nom lisible
  readonly category: AlchemType;      // où il vit dans la classification
  readonly content:  TemplateContent; // le corps du template
  readonly version:  string;
}

export interface TemplateContent {
  readonly code?: string;             // fragment TypeScript généré
  readonly rust?: string;             // fragment Rust équivalent (Luna)
  readonly schema?: Record<string, unknown>; // structure de données
  readonly doc?: string;              // documentation embarquée
}

export type TemplateId = number;
export type TemplateCatalogue = ReadonlyMap<TemplateId, Template>;
