use web_sys::CanvasRenderingContext2d;
use crate::luna::nucleus::Nucleus;
use crate::luna::sprites::Sprites;

pub struct OrbitalRenderer;

impl OrbitalRenderer {
    pub fn draw(
        ctx: &CanvasRenderingContext2d,
        width: f64,
        height: f64,
        nucleus: &Nucleus,
        sprites: &Sprites,
        time: f64,
    ) {
        let cx = width / 2.0;
        let cy = height / 2.0;

        // Background
        ctx.set_fill_style_str("#0a0a0f");
        ctx.fill_rect(0.0, 0.0, width, height);

        // Nucleus glow
        let pulse_r = nucleus.pulse_radius(time);
        let color = nucleus.etat.color();
        if let Ok(grad) = ctx.create_radial_gradient(cx, cy, 0.0, cx, cy, pulse_r * 2.5) {
            let _ = grad.add_color_stop(0.0, color);
            let _ = grad.add_color_stop(0.4, &format!("{}aa", color));
            let _ = grad.add_color_stop(0.7, &format!("{}40", color));
            let _ = grad.add_color_stop(1.0, "transparent");
            ctx.set_fill_style_canvas_gradient(&grad);
            ctx.begin_path();
            let _ = ctx.arc(cx, cy, pulse_r * 2.5, 0.0, std::f64::consts::TAU);
            ctx.fill();
        }

        // Nucleus core
        ctx.set_fill_style_str(color);
        ctx.begin_path();
        let _ = ctx.arc(cx, cy, pulse_r, 0.0, std::f64::consts::TAU);
        ctx.fill();

        // Nucleus label
        ctx.set_fill_style_str("#ffffff");
        ctx.set_font("bold 12px monospace");
        ctx.set_text_align("center");
        let _ = ctx.fill_text("NUCLEUS", cx, cy + 4.0);

        // Orbital paths + sprites
        for sprite in &sprites.list {
            // Dotted orbital path
            ctx.set_stroke_style_str(&format!("{}40", sprite.couleur));
            ctx.set_line_width(1.0);
            ctx.begin_path();
            let _ = ctx.arc(cx, cy, sprite.distance_centre, 0.0, std::f64::consts::TAU);
            ctx.stroke();

            let (sx, sy) = sprite.position(cx, cy);

            // Ghost (quantum superposition)
            if sprite.etat_quantique.is_superpose() {
                let (gx, gy) = sprite.ghost_position(cx, cy);
                ctx.set_global_alpha(0.3);
                ctx.set_fill_style_str(&sprite.couleur);
                ctx.begin_path();
                let _ = ctx.arc(gx, gy, sprite.taille, 0.0, std::f64::consts::TAU);
                ctx.fill();
                ctx.set_global_alpha(1.0);
            }

            // Sprite body
            ctx.set_global_alpha(sprite.opacite);
            if let Ok(grad) = ctx.create_radial_gradient(sx, sy, 0.0, sx, sy, sprite.taille) {
                let _ = grad.add_color_stop(0.0, &sprite.couleur);
                let _ = grad.add_color_stop(0.7, &format!("{}aa", sprite.couleur));
                let _ = grad.add_color_stop(1.0, "transparent");
                ctx.set_fill_style_canvas_gradient(&grad);
            }
            ctx.begin_path();
            let _ = ctx.arc(sx, sy, sprite.taille, 0.0, std::f64::consts::TAU);
            ctx.fill();
            ctx.set_global_alpha(1.0);

            // Sprite label
            ctx.set_fill_style_str("#ffffff");
            ctx.set_font("10px monospace");
            ctx.set_text_align("center");
            let _ = ctx.fill_text(&sprite.nom, sx, sy - sprite.taille - 4.0);
        }
    }
}
