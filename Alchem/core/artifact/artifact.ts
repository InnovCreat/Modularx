// Artifact — Object Data. Plan de construction, pas du code.

import type { TemplateId } from "./template";

export type ArtifactStatus = "prototype" | "in_progress" | "working" | "finished";
export type ArtifactType   = "code" | "documentation" | "module" | "config";

export interface ArtifactHeader {
  readonly artifactId: number;
  readonly name:       string;
  readonly createdBy:  string;
  readonly timestamp:  number;        // Unix ms
  readonly status:     ArtifactStatus;
  readonly type:       ArtifactType;
  readonly frequency?: string;        // ex. "639"
  readonly upgraded?:  boolean;
  readonly parentId?:  number;        // si c'est une mise à niveau
}

export interface Artifact extends ArtifactHeader {
  readonly templates:  readonly TemplateId[];   // référence, pas le code
  readonly tickOrder:  readonly TemplateId[];   // Fixed Sacred Order
  readonly variables:  Readonly<Record<string, unknown>>;
}
