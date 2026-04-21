// Orchestrateur — Le moteur de tick. Fixed Sacred Order. Jamais de magie.

import type { Artifact }          from "../artifact/artifact";
import type { Template, TemplateCatalogue } from "../artifact/template";
import type { ArtifactViews, TreeNode }     from "../artifact/views";

export type TickResult =
  | { ok: true;  views: ArtifactViews }
  | { ok: false; failedAt: number; templateId: number; reason: string };

export function tick(
  artifact: Artifact,
  catalogue: TemplateCatalogue,
): TickResult {
  const resolved: Template[] = [];

  for (let i = 0; i < artifact.tickOrder.length; i++) {
    const id = artifact.tickOrder[i];
    const tmpl = catalogue.get(id);

    if (!tmpl) {
      return { ok: false, failedAt: i, templateId: id, reason: `Template ${id} not found in catalogue` };
    }

    resolved.push(tmpl);
  }

  return {
    ok: true,
    views: buildViews(artifact, resolved),
  };
}

function buildViews(artifact: Artifact, templates: Template[]): ArtifactViews {
  return {
    code: {
      header: artifact,
      code:   templates.map(t => t.content.code ?? "").filter(Boolean).join("\n"),
    },
    tree: {
      header: artifact,
      root: {
        templateId:   0,
        templateName: artifact.name,
        children:     templates.map(toNode),
      },
    },
    text: {
      header:      artifact,
      explanation: templates.map(t => t.content.doc ?? "").filter(Boolean).join(" "),
    },
  };
}

function toNode(t: Template): TreeNode {
  return { templateId: t.id, templateName: t.name, children: [] };
}
