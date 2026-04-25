use bevy::prelude::*;
use super::geometry::PHI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlatonicSolid {
    Tetrahedron,  // 720 Hz
    Cube,         // 1440 Hz
    Octahedron,   // 2160 Hz
    Dodecahedron, // 3600 Hz
    Icosahedron,  // 6480 Hz
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

    pub fn vertices(self) -> Vec<Vec3> {
        match self {
            Self::Tetrahedron  => tetrahedron_verts(),
            Self::Cube         => cube_verts(),
            Self::Octahedron   => octahedron_verts(),
            Self::Dodecahedron => dodecahedron_verts(),
            Self::Icosahedron  => icosahedron_verts(),
        }
    }
}

fn tetrahedron_verts() -> Vec<Vec3> {
    vec![
        Vec3::new( 1.0,  1.0,  1.0),
        Vec3::new( 1.0, -1.0, -1.0),
        Vec3::new(-1.0,  1.0, -1.0),
        Vec3::new(-1.0, -1.0,  1.0),
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
    v
}

fn octahedron_verts() -> Vec<Vec3> {
    vec![
        Vec3::X, Vec3::NEG_X,
        Vec3::Y, Vec3::NEG_Y,
        Vec3::Z, Vec3::NEG_Z,
    ]
}

fn dodecahedron_verts() -> Vec<Vec3> {
    let inv = 1.0 / PHI;
    let mut v = Vec::with_capacity(20);
    for sx in [-1.0_f32, 1.0] {
        for sy in [-1.0_f32, 1.0] {
            for sz in [-1.0_f32, 1.0] {
                v.push(Vec3::new(sx, sy, sz));
            }
        }
    }
    for s1 in [-1.0_f32, 1.0] {
        for s2 in [-1.0_f32, 1.0] {
            v.push(Vec3::new(0.0,       s1 * inv, s2 * PHI));
            v.push(Vec3::new(s1 * inv,  s2 * PHI, 0.0));
            v.push(Vec3::new(s1 * PHI,  0.0,      s2 * inv));
        }
    }
    v
}

fn icosahedron_verts() -> Vec<Vec3> {
    let mut v = Vec::with_capacity(12);
    for s1 in [-1.0_f32, 1.0] {
        for s2 in [-1.0_f32, 1.0] {
            v.push(Vec3::new(0.0,      s1,        s2 * PHI));
            v.push(Vec3::new(s1,       s2 * PHI,  0.0));
            v.push(Vec3::new(s1 * PHI, 0.0,       s2));
        }
    }
    v
}

#[derive(Resource, Default)]
pub struct PlatonicRegistry {
    pub active: Option<PlatonicSolid>,
}
