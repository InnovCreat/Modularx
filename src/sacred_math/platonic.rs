// ─────────────────────────────────────────────
// VERITAS HORTUS · sacred_math · platonic.rs
// Architecte  : Isabel Sigouin (InnovCreat)
// Registre    : Mathématique
// Fonction    : Comprendre
// Gravité     : Forme
// Révolution  : interne
// Souveraineté: nulle
// Chaîne      : ─
// Covenant    : Jamais guerre · Jamais cupidité
//               Toujours connaissance · Toujours amour
// Constitution v1.0 — Article IV · Langage v1.4 — Deux Centres
// ─────────────────────────────────────────────

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
};
use super::geometry::PHI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlatonicSolid {
    Tetrahedron,  // 720 Hz  | 4 faces
    Cube,         // 1440 Hz | 6 faces
    Octahedron,   // 2160 Hz | 8 faces
    Dodecahedron, // 3600 Hz | 12 faces
    Icosahedron,  // 6480 Hz | 20 faces
}

impl PlatonicSolid {
    pub const ALL: [Self; 5] = [
        Self::Tetrahedron,
        Self::Cube,
        Self::Octahedron,
        Self::Dodecahedron,
        Self::Icosahedron,
    ];

    pub fn frequency(self) -> f32 {
        match self {
            Self::Tetrahedron  => 720.0,
            Self::Cube         => 1440.0,
            Self::Octahedron   => 2160.0,
            Self::Dodecahedron => 3600.0,
            Self::Icosahedron  => 6480.0,
        }
    }

    pub fn dual(self) -> Self {
        match self {
            Self::Tetrahedron  => Self::Tetrahedron,
            Self::Cube         => Self::Octahedron,
            Self::Octahedron   => Self::Cube,
            Self::Dodecahedron => Self::Icosahedron,
            Self::Icosahedron  => Self::Dodecahedron,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Tetrahedron  => "Tétraèdre",
            Self::Cube         => "Cube",
            Self::Octahedron   => "Octaèdre",
            Self::Dodecahedron => "Dodécaèdre",
            Self::Icosahedron  => "Icosaèdre",
        }
    }

    /// Math vertices (un-normalized) — for causality/geometry computations
    pub fn vertices(self) -> Vec<Vec3> {
        match self {
            Self::Tetrahedron  => tetra_verts(),
            Self::Cube         => cube_verts(),
            Self::Octahedron   => octa_verts(),
            Self::Dodecahedron => dodeca_verts(),
            Self::Icosahedron  => icosa_verts(),
        }
    }

    /// Renderable Bevy Mesh with flat shading + outward normals
    pub fn build_mesh(self) -> Mesh {
        match self {
            Self::Tetrahedron  => build_flat_mesh(&tetra_verts(),  &tetra_faces()),
            Self::Cube         => build_flat_mesh(&cube_verts(),   &cube_faces()),
            Self::Octahedron   => build_flat_mesh(&octa_verts(),   &octa_faces()),
            Self::Dodecahedron => build_flat_mesh(&dodeca_verts(), &dodeca_faces()),
            Self::Icosahedron  => build_flat_mesh(&icosa_verts(),  &icosa_faces()),
        }
    }
}

// ── Vertices ──────────────────────────────────────────────────────────────────

fn tetra_verts() -> Vec<Vec3> {
    vec![
        Vec3::new( 1.0,  1.0,  1.0), // 0
        Vec3::new( 1.0, -1.0, -1.0), // 1
        Vec3::new(-1.0,  1.0, -1.0), // 2
        Vec3::new(-1.0, -1.0,  1.0), // 3
    ]
}

fn cube_verts() -> Vec<Vec3> {
    let mut v = Vec::with_capacity(8);
    for x in [-1.0_f32, 1.0] {
        for y in [-1.0_f32, 1.0] {
            for z in [-1.0_f32, 1.0] {
                v.push(Vec3::new(x, y, z));
            }
        }
    }
    // order: 0=(-1,-1,-1) 1=(-1,-1,1) 2=(-1,1,-1) 3=(-1,1,1)
    //        4=(1,-1,-1)  5=(1,-1,1)  6=(1,1,-1)  7=(1,1,1)
    v
}

fn octa_verts() -> Vec<Vec3> {
    // 0=+X 1=-X 2=+Y 3=-Y 4=+Z 5=-Z
    vec![Vec3::X, Vec3::NEG_X, Vec3::Y, Vec3::NEG_Y, Vec3::Z, Vec3::NEG_Z]
}

fn dodeca_verts() -> Vec<Vec3> {
    let inv = 1.0 / PHI;
    // 8 cube corners
    let mut v: Vec<Vec3> = Vec::with_capacity(20);
    for x in [-1.0_f32, 1.0] {
        for y in [-1.0_f32, 1.0] {
            for z in [-1.0_f32, 1.0] {
                v.push(Vec3::new(x, y, z));
            }
        }
    }
    // v0=(-1,-1,-1) v1=(-1,-1,1) v2=(-1,1,-1) v3=(-1,1,1)
    // v4=(1,-1,-1)  v5=(1,-1,1)  v6=(1,1,-1)  v7=(1,1,1)
    //
    // 4 rect (0, ±1/φ, ±φ)
    // v8=(0,-inv,-PHI) v9=(0,-inv,PHI) v10=(0,inv,-PHI) v11=(0,inv,PHI)
    for y in [-inv, inv] {
        for z in [-PHI, PHI] {
            v.push(Vec3::new(0.0, y, z));
        }
    }
    // 4 rect (±1/φ, ±φ, 0)
    // v12=(-inv,-PHI,0) v13=(-inv,PHI,0) v14=(inv,-PHI,0) v15=(inv,PHI,0)
    for x in [-inv, inv] {
        for y in [-PHI, PHI] {
            v.push(Vec3::new(x, y, 0.0));
        }
    }
    // 4 rect (±φ, 0, ±1/φ)
    // v16=(-PHI,0,-inv) v17=(-PHI,0,inv) v18=(PHI,0,-inv) v19=(PHI,0,inv)
    for x in [-PHI, PHI] {
        for z in [-inv, inv] {
            v.push(Vec3::new(x, 0.0, z));
        }
    }
    v
}

fn icosa_verts() -> Vec<Vec3> {
    // Standard icosahedron vertex ordering with known face indices
    vec![
        Vec3::new(-1.0,  PHI,  0.0), // 0
        Vec3::new( 1.0,  PHI,  0.0), // 1
        Vec3::new(-1.0, -PHI,  0.0), // 2
        Vec3::new( 1.0, -PHI,  0.0), // 3
        Vec3::new( 0.0, -1.0,  PHI), // 4
        Vec3::new( 0.0,  1.0,  PHI), // 5
        Vec3::new( 0.0, -1.0, -PHI), // 6
        Vec3::new( 0.0,  1.0, -PHI), // 7
        Vec3::new( PHI,  0.0, -1.0), // 8
        Vec3::new( PHI,  0.0,  1.0), // 9
        Vec3::new(-PHI,  0.0, -1.0), // 10
        Vec3::new(-PHI,  0.0,  1.0), // 11
    ]
}

// ── Face index tables ─────────────────────────────────────────────────────────

fn tetra_faces() -> Vec<[usize; 3]> {
    vec![[0,1,2], [0,3,1], [0,2,3], [1,3,2]]
}

fn cube_faces() -> Vec<[usize; 3]> {
    // 6 faces × 2 triangles = 12 (using vertex map from cube_verts)
    vec![
        [0,4,5], [0,5,1], // bottom  y=-1
        [2,3,7], [2,7,6], // top     y=+1
        [1,5,7], [1,7,3], // front   z=+1
        [4,0,2], [4,2,6], // back    z=-1
        [0,1,3], [0,3,2], // left    x=-1
        [5,4,6], [5,6,7], // right   x=+1
    ]
}

fn octa_faces() -> Vec<[usize; 3]> {
    // top cap (y=+1 = vertex 2), bottom cap (y=-1 = vertex 3)
    vec![
        [2,0,4], [2,4,1], [2,1,5], [2,5,0], // top
        [3,4,0], [3,1,4], [3,5,1], [3,0,5], // bottom
    ]
}

fn dodeca_faces() -> Vec<[usize; 3]> {
    // 12 pentagons (derived by hand from adjacency graph, edge_length = 2/φ),
    // fan-triangulated from first vertex → 3 triangles per face = 36 total
    let pentagons: [[usize; 5]; 12] = [
        [ 9,  1, 17,  3, 11], // top    +z
        [ 8,  0, 16,  2, 10], // bottom -z
        [12,  0,  8,  4, 14], // front- -y left
        [14,  5,  9,  1, 12], // front+ -y right
        [19,  5, 14,  4, 18], // right  +x bottom
        [11,  7, 19,  5,  9], // right  +x top
        [ 7, 15, 13,  3, 11], // top    +y left
        [15,  6, 10,  2, 13], // back   -z top
        [16,  2, 13,  3, 17], // left   -x top
        [17,  1, 12,  0, 16], // left   -x bottom
        [18,  6, 10,  8,  4], // back   -z bottom
        [ 6, 15,  7, 19, 18], // right  +x back
    ];
    let mut tris = Vec::with_capacity(36);
    for p in &pentagons {
        // fan from p[0]
        tris.push([p[0], p[1], p[2]]);
        tris.push([p[0], p[2], p[3]]);
        tris.push([p[0], p[3], p[4]]);
    }
    tris
}

fn icosa_faces() -> Vec<[usize; 3]> {
    // Standard 20 faces for the icosahedron vertex ordering in icosa_verts()
    vec![
        // 5 faces around vertex 0
        [0,11, 5], [0, 5, 1], [0, 1, 7], [0, 7,10], [0,10,11],
        // 5 adjacent faces
        [1, 5, 9], [5,11, 4], [11,10, 2], [10, 7, 6], [7, 1, 8],
        // 5 faces around vertex 3
        [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9],
        // 5 faces adjacent to 3
        [4, 9, 5], [2, 4,11], [6, 2,10], [8, 6, 7], [9, 8, 1],
    ]
}

// ── Mesh builder ──────────────────────────────────────────────────────────────

fn build_flat_mesh(verts: &[Vec3], faces: &[[usize; 3]]) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(faces.len() * 3);
    let mut normals:   Vec<[f32; 3]> = Vec::with_capacity(faces.len() * 3);
    let mut indices:   Vec<u32>      = Vec::with_capacity(faces.len() * 3);

    for (i, &[ai, bi, ci]) in faces.iter().enumerate() {
        let a = verts[ai];
        let b = verts[bi];
        let c = verts[ci];
        let centroid = (a + b + c) / 3.0;

        // Compute raw normal; flip winding if it points inward
        let n_raw = (b - a).cross(c - a);
        let (pa, pb, pc, n) = if n_raw.dot(centroid) >= 0.0 {
            (a, b, c, n_raw.normalize())
        } else {
            (a, c, b, (-n_raw).normalize()) // reverse winding + flip normal
        };

        let ni = [n.x, n.y, n.z];
        let base = (i * 3) as u32;
        positions.extend([[pa.x,pa.y,pa.z],[pb.x,pb.y,pb.z],[pc.x,pc.y,pc.z]]);
        normals.extend([ni, ni, ni]);
        indices.extend([base, base+1, base+2]);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

// ── Resource ──────────────────────────────────────────────────────────────────

#[derive(Resource)]
pub struct PlatonicRegistry {
    pub active: PlatonicSolid,
}

impl Default for PlatonicRegistry {
    fn default() -> Self {
        Self { active: PlatonicSolid::Icosahedron }
    }
}
