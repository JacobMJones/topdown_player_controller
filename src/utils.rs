// utils.rs

use ggez::graphics::Color;

pub fn get_dynamic_color(time: f32, normalized_distance: f32, in_proximity: bool) -> Color {
    let check: f32 = 0.6 + (normalized_distance * 10.0);

    if !in_proximity {
        let g = ((time.sin() * 0.25 + 0.75) * 0.5 + 0.5) as f32;
        let b = 0.0 as f32;
        let r = 1.0 as f32;

        Color::new(r, g, b, 1.0)
    } else {
        //close
        if check < 7.0 {
            let g = ((time.sin() * 0.25 + 0.75) * 0.5 + 0.5) as f32;
            let b = 1.0 as f32;
            let r = 0.0;
            Color::new(r, g, b, 0.9)
        //very close
        } else {
            let g = 0.5;
            let b = 0.5 as f32;
            let r = 1.0 as f32;
            Color::new(r, g, b, 0.6)
        }
    }
}


pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}