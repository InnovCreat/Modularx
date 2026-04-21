// Alchem — Utilitaires
// Fonctions pures uniquement. Aucun état, aucun side effect.

export function makeId(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 7)}`;
}

export function isCapacity(name: string): boolean {
  return name.startsWith("Can");
}

export function classify(name: string): "Capacity" | "unknown" {
  if (isCapacity(name)) return "Capacity";
  return "unknown";
}
