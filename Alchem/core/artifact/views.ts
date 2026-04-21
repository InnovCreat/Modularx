// ArtifactViews — Les 3 vues d'un artefact reconstruit.

import type { ArtifactHeader } from "./artifact";

export interface CodeView {
  readonly header:  ArtifactHeader;
  readonly code:    string;           // code généré, prêt à exécuter
}

export interface TreeNode {
  readonly templateId:   number;
  readonly templateName: string;
  readonly children:     readonly TreeNode[];
}

export interface TreeView {
  readonly header: ArtifactHeader;
  readonly root:   TreeNode;
}

export interface TextView {
  readonly header:       ArtifactHeader;
  readonly explanation:  string;      // langage humain, accessible à tous
}

export interface ArtifactViews {
  readonly code: CodeView;
  readonly tree: TreeView;
  readonly text: TextView;
}
