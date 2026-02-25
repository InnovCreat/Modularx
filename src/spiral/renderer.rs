use web_sys::CanvasRenderingContext2d;
use super::data::SpiralData;

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub radius: f64,
    pub color: String,
}

pub struct SpiralRenderer {
    pub data: SpiralData,
    pub particles: Vec<Particle>,
}

impl SpiralRenderer {
    pub fn new() -> Self {
        let data = SpiralData::default_data();
        let mut particles = Vec::new();
        for i in 0..30 {
            let ring_idx = i % data.rings.len();
            particles.push(Particle {
                x: pseudo_random(i as f64 * 13.37) * 1200.0,
                y: pseudo_random(i as f64 * 7.42) * 800.0,
                vx: (pseudo_random(i as f64 * 3.14) - 0.5) * 0.5,
                vy: (pseudo_random(i as f64 * 2.71) - 0.5) * 0.5,
                radius: pseudo_random(i as f64 * 1.61) * 2.0 + 1.0,
                color: data.rings[ring_idx].color.to_string(),
            });
        }
        Self { data, particles }
    }

    pub fn draw(
        &mut self,
        ctx: &CanvasRenderingContext2d,
        width: f64,
        height: f64,
        rotation: f64,
        zoom: f64,
        frequency: f64,
        selected_ring: Option<usize>,
        show_labels: bool,
        show_particles: bool,
        time: f64,
    ) {
        let cx = width / 2.0;
        let cy = height / 2.0;

        // Background
        ctx.set_fill_style_str("#0a0a0f");
        ctx.fill_rect(0.0, 0.0, width, height);

        // Particles
        if show_particles {
            for p in &mut self.particles {
                p.x += p.vx;
                p.y += p.vy;
                if p.x < 0.0 || p.x > width { p.vx *= -1.0; }
                if p.y < 0.0 || p.y > height { p.vy *= -1.0; }

                ctx.set_fill_style_str(&format!("{}40", p.color));
                ctx.begin_path();
                let _ = ctx.arc(p.x, p.y, p.radius, 0.0, std::f64::consts::TAU);
                ctx.fill();
            }
        }

        // Center pulse
        let pulse = 15.0 + (time * frequency / 100.0).sin() * 8.0;
        let center_color = self.data.center.color;
        if let Ok(grad) = ctx.create_radial_gradient(cx, cy, 0.0, cx, cy, pulse * zoom * 2.0) {
            let _ = grad.add_color_stop(0.0, center_color);
            let _ = grad.add_color_stop(0.4, &format!("{}aa", center_color));
            let _ = grad.add_color_stop(0.7, &format!("{}40", center_color));
            let _ = grad.add_color_stop(1.0, "transparent");
            ctx.set_fill_style_canvas_gradient(&grad);
            ctx.begin_path();
            let _ = ctx.arc(cx, cy, pulse * zoom * 2.0, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }

        // Center label
        if show_labels {
            ctx.set_fill_style_str("#ffffff");
            ctx.set_font("bold 14px monospace");
            ctx.set_text_align("center");
            let _ = ctx.fill_text(self.data.center.name, cx, cy + 5.0);
        }

        // Spiral rings
        let rings = self.data.rings.clone();
        for (ring_idx, ring) in rings.iter().enumerate() {
            let radius = (60.0 + ring_idx as f64 * 50.0) * zoom;
            let is_selected = selected_ring == Some(ring_idx);
            let points = 128;

            // Connection lines to center
            if is_selected {
                ctx.set_stroke_style_str(&format!("{}40", ring.color));
                ctx.set_line_width(1.0);
                for c_idx in 0..ring.concepts.len() {
                    let angle = (c_idx as f64 / ring.concepts.len() as f64) * std::f64::consts::TAU + rotation;
                    let x = cx + angle.cos() * radius;
                    let y = cy + angle.sin() * radius;
                    ctx.begin_path();
                    ctx.move_to(cx, cy);
                    ctx.line_to(x, y);
                    ctx.stroke();
                }
            }

            // Main spiral wave
            let alpha = if is_selected { "ff" } else { "80" };
            ctx.set_stroke_style_str(&format!("{}{}", ring.color, alpha));
            ctx.set_line_width(if is_selected { 3.0 } else { 2.0 });
            ctx.set_shadow_blur(if is_selected { 20.0 } else { 0.0 });
            ctx.set_shadow_color(ring.color);
            ctx.begin_path();

            for i in 0..=points {
                let angle = (i as f64 / points as f64) * std::f64::consts::TAU
                    + rotation
                    + ring_idx as f64 * 0.3;
                let wave = (i as f64 * 0.15 + time).sin() * 12.0 * zoom;
                let r = radius + wave;
                let x = cx + angle.cos() * r;
                let y = cy + angle.sin() * r;
                if i == 0 { ctx.move_to(x, y); } else { ctx.line_to(x, y); }
            }
            ctx.stroke();
            ctx.set_shadow_blur(0.0);

            // Concept points
            for (c_idx, concept) in ring.concepts.iter().enumerate() {
                let angle = (c_idx as f64 / ring.concepts.len() as f64) * std::f64::consts::TAU + rotation;
                let x = cx + angle.cos() * radius;
                let y = cy + angle.sin() * radius;

                if let Ok(pg) = ctx.create_radial_gradient(x, y, 0.0, x, y, 8.0 * zoom) {
                    let _ = pg.add_color_stop(0.0, ring.color);
                    let _ = pg.add_color_stop(0.5, &format!("{}aa", ring.color));
                    let _ = pg.add_color_stop(1.0, "transparent");
                    ctx.set_fill_style_canvas_gradient(&pg);
                    ctx.begin_path();
                    let _ = ctx.arc(x, y, 8.0 * zoom, 0.0, std::f64::consts::TAU);
                    ctx.fill();
                }

                if show_labels && (is_selected || ring_idx < 3) {
                    ctx.set_fill_style_str("#ffffff");
                    let font_size = 10.0 * zoom;
                    let bold = if is_selected { "bold " } else { "" };
                    ctx.set_font(&format!("{}{}px monospace", bold, font_size));
                    ctx.set_text_align("center");
                    let _ = ctx.fill_text(concept, x, y - 15.0 * zoom);
                }
            }

            // Theme label (selected)
            if show_labels && is_selected {
                let la = rotation + std::f64::consts::FRAC_PI_4;
                let lx = cx + la.cos() * (radius + 30.0 * zoom);
                let ly = cy + la.sin() * (radius + 30.0 * zoom);
                ctx.set_fill_style_str(ring.color);
                ctx.set_font("bold 16px monospace");
                ctx.set_text_align("center");
                let _ = ctx.fill_text(&ring.theme.to_uppercase(), lx, ly);
            }
        }
    }
}

fn pseudo_random(seed: f64) -> f64 {
    let x = (seed * 12.9898 + 78.233).sin() * 43758.5453;
    x - x.floor()
}
