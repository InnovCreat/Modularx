// Alchem — Source unique de vérité pour la configuration
// Toute valeur de config vient d'ici. Jamais dispersée dans les modules.

export const SETTINGS = Object.freeze({
  bus: {
    maxListeners: 100,
    logEmits:     false,
  },
  module: {
    requireModuleMd: true,   // chaque module doit avoir module.md
    strictBoundary:  true,   // un module ne voit pas l'intérieur d'un autre
  },
  classification: {
    enforceFileSuffix: false, // guide, pas loi — à activer si outillage prêt
  },
} as const);

export type Settings = typeof SETTINGS;
