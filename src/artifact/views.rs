use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CodeView {
    pub artifact_id: u32,
    pub name:        String,
    pub code:        String,   // fragments Rust assemblés dans l'ordre du tick
}

#[derive(Debug, Clone, Serialize)]
pub struct TreeNode {
    pub template_id:   u32,
    pub template_name: String,
    pub children:      Vec<TreeNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TreeView {
    pub artifact_id: u32,
    pub name:        String,
    pub root:        TreeNode,
}

#[derive(Debug, Clone, Serialize)]
pub struct TextView {
    pub artifact_id: u32,
    pub name:        String,
    pub explanation: String,   // docs concatenées dans l'ordre du tick
}

#[derive(Debug, Clone, Serialize)]
pub struct ArtifactViews {
    pub code: CodeView,
    pub tree: TreeView,
    pub text: TextView,
}
