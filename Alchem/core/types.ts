// Alchem — Types partagés du framework
// Object Data uniquement. Aucune logique.

export type ModuleState = "active" | "latent" | "error";

export interface ModuleIdentity {
  readonly name:    string;
  readonly version: string;
  readonly layer:   "core" | "interface" | "module";
}

export interface ClassifiedSymbol {
  readonly key:      string;
  readonly category: "Identity" | "Action" | "Capacity" | "Space" | "Control";
  readonly alchemType: "Object Data" | "Command Data" | "Ability" | "Environment" | "Security";
}
