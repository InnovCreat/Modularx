//! CRYSTAL ASCII 3D — NEXUS SOVEREIGN RENDERER
//! Zero-dependency terminal visualization | VERITAS HORTUS
//! Fréquence: 639Hz | Philosophie: Jamais pour la guerre

use std::io::{self, Write, stdout};
use std::time::{Duration, Instant};
use std::f32::consts::PI;

// ═══════════════════════════════════════════════════════════════════
// MATHEMATICAL PRIMITIVES
// ═══════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    fn rotate_y(&self, angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Self {
            x: self.x * cos_a + self.z * sin_a,
            y: self.y,
            z: -self.x * sin_a + self.z * cos_a,
        }
    }

    fn rotate_x(&self, angle: f32) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Self {
            x: self.x,
            y: self.y * cos_a - self.z * sin_a,
            z: self.y * sin_a + self.z * cos_a,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// CAMERA SYSTEM
// ═══════════════════════════════════════════════════════════════════

struct Camera {
    distance: f32,
    rot_y: f32,
    rot_x: f32,
    fov: f32,
}

impl Camera {
    fn new() -> Self {
        Self {
            distance: 5.0,
            rot_y: 0.0,
            rot_x: 0.3,
            fov: 90.0,
        }
    }

    fn project(&self, point: &Vec3, width: usize, height: usize) -> Option<(usize, usize, f32)> {
        let rotated = point.rotate_y(-self.rot_y).rotate_x(-self.rot_x);
        let z = rotated.z + self.distance;

        if z <= 0.1 { return None; }

        let fov_rad = self.fov * PI / 180.0;
        let scale = (fov_rad / 2.0).tan();
        let aspect = width as f32 / height as f32 * 0.5;

        let px = (rotated.x / z / scale) * aspect;
        let py = rotated.y / z / scale;

        let sx = ((px + 1.0) * 0.5 * width as f32) as i32;
        let sy = ((1.0 - py) * 0.5 * height as f32) as i32;

        if sx >= 0 && sx < width as i32 && sy >= 0 && sy < height as i32 {
            Some((sx as usize, sy as usize, z))
        } else {
            None
        }
    }
}

// ═══════════════════════════════════════════════════════════════════
// ASCII FRAME BUFFER
// ═══════════════════════════════════════════════════════════════════

struct FrameBuffer {
    width: usize,
    height: usize,
    chars: Vec<char>,
    depth: Vec<f32>,
}

impl FrameBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            chars: vec![' '; width * height],
            depth: vec![f32::MAX; width * height],
        }
    }

    fn clear(&mut self) {
        self.chars.fill(' ');
        self.depth.fill(f32::MAX);
    }

    fn set(&mut self, x: usize, y: usize, c: char, z: f32) {
        let idx = y * self.width + x;
        if idx < self.chars.len() && z < self.depth[idx] {
            self.depth[idx] = z;
            self.chars[idx] = c;
        }
    }

    fn draw_line(&mut self, x0: i32, y0: i32, z0: f32, x1: i32, y1: i32, z1: f32, c: char) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;

        let steps = dx.max(dy).max(1) as f32;
        let mut t = 0.0;

        loop {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                let z = z0 + (z1 - z0) * t;
                self.set(x as usize, y as usize, c, z);
            }

            if x == x1 && y == y1 { break; }

            let e2 = 2 * err;
            if e2 > -dy { err -= dy; x += sx; }
            if e2 < dx { err += dx; y += sy; }
            t += 1.0 / steps;
        }
    }

    fn render(&self) -> String {
        let mut output = String::with_capacity((self.width + 1) * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                output.push(self.chars[y * self.width + x]);
            }
            output.push('\n');
        }
        output
    }
}

// ═══════════════════════════════════════════════════════════════════
// CRYSTAL GEOMETRY
// ═══════════════════════════════════════════════════════════════════

struct Crystal {
    vertices: Vec<Vec3>,
    edges: Vec<(usize, usize)>,
    rotation: Vec3,
    position: Vec3,
}

impl Crystal {
    fn icosahedron(scale: f32) -> Self {
        let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;

        let vertices = vec![
            Vec3::new(-1.0,  phi,  0.0), Vec3::new( 1.0,  phi,  0.0),
            Vec3::new(-1.0, -phi,  0.0), Vec3::new( 1.0, -phi,  0.0),
            Vec3::new( 0.0, -1.0,  phi), Vec3::new( 0.0,  1.0,  phi),
            Vec3::new( 0.0, -1.0, -phi), Vec3::new( 0.0,  1.0, -phi),
            Vec3::new( phi,  0.0, -1.0), Vec3::new( phi,  0.0,  1.0),
            Vec3::new(-phi,  0.0, -1.0), Vec3::new(-phi,  0.0,  1.0),
        ].into_iter().map(|v| Vec3::new(v.x * scale, v.y * scale, v.z * scale)).collect();

        let edges = vec![
            (0,1), (0,5), (0,7), (0,10), (0,11),
            (1,5), (1,7), (1,8), (1,9),
            (2,3), (2,4), (2,6), (2,10), (2,11),
            (3,4), (3,6), (3,8), (3,9),
            (4,5), (4,9), (4,11), (5,9), (5,11),
            (6,7), (6,8), (6,10), (7,8), (7,10),
            (8,9), (10,11),
        ];

        Self { vertices, edges, rotation: Vec3::new(0.0, 0.0, 0.0), position: Vec3::new(0.0, 0.0, 0.0) }
    }

    fn octahedron(scale: f32) -> Self {
        let vertices = vec![
            Vec3::new( 1.0,  0.0,  0.0), Vec3::new(-1.0,  0.0,  0.0),
            Vec3::new( 0.0,  1.0,  0.0), Vec3::new( 0.0, -1.0,  0.0),
            Vec3::new( 0.0,  0.0,  1.0), Vec3::new( 0.0,  0.0, -1.0),
        ].into_iter().map(|v| Vec3::new(v.x * scale, v.y * scale, v.z * scale)).collect();

        let edges = vec![
            (0,2), (0,3), (0,4), (0,5),
            (1,2), (1,3), (1,4), (1,5),
            (2,4), (2,5), (3,4), (3,5),
        ];

        Self { vertices, edges, rotation: Vec3::new(0.0, 0.0, 0.0), position: Vec3::new(0.0, 0.0, 0.0) }
    }

    fn transform(&self) -> Vec<Vec3> {
        self.vertices.iter().map(|v| {
            let rotated = v.rotate_x(self.rotation.x).rotate_y(self.rotation.y);
            Vec3::new(
                rotated.x + self.position.x,
                rotated.y + self.position.y,
                rotated.z + self.position.z,
            )
        }).collect()
    }
}

// ═══════════════════════════════════════════════════════════════════
// NEXUS SCENE
// ═══════════════════════════════════════════════════════════════════

struct NexusScene {
    central_crystal: Crystal,
    orbital_crystals: Vec<Crystal>,
    orbital_angle: f32,
    ring_segments: Vec<Vec3>,
}

impl NexusScene {
    fn new() -> Self {
        let mut orbital_crystals = Vec::new();
        for i in 0..8 {
            let angle = (i as f32 / 8.0) * 2.0 * PI;
            let mut crystal = Crystal::icosahedron(0.3);
            crystal.position = Vec3::new(angle.cos() * 2.5, 0.0, angle.sin() * 2.5);
            orbital_crystals.push(crystal);
        }

        let ring_segments: Vec<Vec3> = (0..48).map(|i| {
            let angle = (i as f32 / 48.0) * 2.0 * PI;
            Vec3::new(1.2 * angle.cos(), 0.0, 1.2 * angle.sin())
        }).collect();

        Self {
            central_crystal: Crystal::octahedron(0.6),
            orbital_crystals,
            orbital_angle: 0.0,
            ring_segments,
        }
    }

    fn update(&mut self, dt: f32) {
        self.central_crystal.rotation.y += dt * 0.8;
        self.central_crystal.rotation.x += dt * 0.5;

        self.orbital_angle += dt * 0.3;
        for (i, crystal) in self.orbital_crystals.iter_mut().enumerate() {
            let base_angle = (i as f32 / 8.0) * 2.0 * PI;
            let angle = base_angle + self.orbital_angle;
            crystal.position.x = angle.cos() * 2.5;
            crystal.position.z = angle.sin() * 2.5;
            crystal.rotation.y += dt * 1.2;
            crystal.rotation.x += dt * 0.7;
        }
    }

    fn render(&self, buffer: &mut FrameBuffer, camera: &Camera) {
        // Gyroscope rings
        for rot in [(0.0, 0.0), (PI/2.0, 0.0), (0.0, PI/2.0)] {
            self.render_ring(buffer, camera, rot.0, rot.1);
        }

        // Orbital crystals
        for crystal in &self.orbital_crystals {
            self.render_crystal(buffer, camera, crystal, '\u{25C7}');
        }

        // Central crystal
        self.render_crystal(buffer, camera, &self.central_crystal, '\u{25C6}');
    }

    fn render_crystal(&self, buffer: &mut FrameBuffer, camera: &Camera, crystal: &Crystal, vertex_char: char) {
        let transformed = crystal.transform();
        let w = buffer.width;
        let h = buffer.height;

        let projected: Vec<Option<(usize, usize, f32)>> = transformed.iter()
            .map(|v| camera.project(v, w, h))
            .collect();

        for &(i, j) in &crystal.edges {
            if let (Some((x0, y0, z0)), Some((x1, y1, z1))) = (projected[i], projected[j]) {
                let edge_char = self.edge_char(z0);
                buffer.draw_line(x0 as i32, y0 as i32, z0, x1 as i32, y1 as i32, z1, edge_char);
            }
        }

        for proj in &projected {
            if let Some((x, y, z)) = proj {
                buffer.set(*x, *y, vertex_char, *z - 0.01);
            }
        }
    }

    fn render_ring(&self, buffer: &mut FrameBuffer, camera: &Camera, rot_x: f32, rot_y: f32) {
        let w = buffer.width;
        let h = buffer.height;

        let rotated: Vec<Vec3> = self.ring_segments.iter()
            .map(|v| v.rotate_x(rot_x).rotate_y(rot_y))
            .collect();

        for i in 0..rotated.len() {
            let j = (i + 1) % rotated.len();
            if let (Some((x0, y0, z0)), Some((x1, y1, z1))) =
                (camera.project(&rotated[i], w, h), camera.project(&rotated[j], w, h)) {
                buffer.draw_line(x0 as i32, y0 as i32, z0, x1 as i32, y1 as i32, z1, '\u{00B7}');
            }
        }
    }

    fn edge_char(&self, depth: f32) -> char {
        if depth < 4.0 { '\u{2588}' }
        else if depth < 5.0 { '\u{2593}' }
        else if depth < 6.0 { '\u{2592}' }
        else if depth < 7.0 { '\u{2591}' }
        else { '\u{00B7}' }
    }
}

// ═══════════════════════════════════════════════════════════════════
// TERMINAL CONTROL
// ═══════════════════════════════════════════════════════════════════

fn enable_raw_mode() {
    print!("\x1B[?1049h\x1B[?25l\x1B[2J");
    let _ = stdout().flush();
}

fn disable_raw_mode() {
    print!("\x1B[?25h\x1B[?1049l");
    let _ = stdout().flush();
}

// ═══════════════════════════════════════════════════════════════════
// MAIN
// ═══════════════════════════════════════════════════════════════════

fn main() {
    let width = 100;
    let height = 35;

    enable_raw_mode();

    let mut buffer = FrameBuffer::new(width, height);
    let mut camera = Camera::new();
    let mut scene = NexusScene::new();
    let mut last_time = Instant::now();
    let frame_duration = Duration::from_millis(33);

    let border_top = format!("\u{2554}{:\u{2550}<width$}\u{2557}", "", width = width);
    let border_bot = format!("\u{255A}{:\u{2550}<width$}\u{255D}", "", width = width);

    for frame in 0..300 { // ~10 seconds
        let now = Instant::now();
        let dt = (now - last_time).as_secs_f32();
        last_time = now;

        scene.update(dt);
        camera.rot_y += dt * 0.15;

        buffer.clear();
        scene.render(&mut buffer, &camera);

        print!("\x1B[H");
        println!("\x1B[36m{}\x1B[0m", border_top);
        println!("\x1B[96m\u{2551} CRYSTAL ASCII 3D \u{2014} NEXUS                          VERITAS HORTUS | 639Hz \u{2551}\x1B[0m");

        for line in buffer.render().lines() {
            println!("\x1B[36m\u{2551}\x1B[97m{:<width$}\x1B[36m\u{2551}\x1B[0m", line, width = width);
        }

        println!("\x1B[36m{}\x1B[0m", border_bot);
        println!("\x1B[90m  Frame {} | Orbital: {:.1}\u{00B0} | Camera Y: {:.1}\u{00B0}\x1B[0m",
                 frame, scene.orbital_angle.to_degrees() % 360.0, camera.rot_y.to_degrees() % 360.0);

        let _ = stdout().flush();
        std::thread::sleep(frame_duration.saturating_sub(now.elapsed()));
    }

    disable_raw_mode();
    println!("\nNEXUS session complete.");
}
