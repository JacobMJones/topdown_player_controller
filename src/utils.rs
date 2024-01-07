// utils.rs

use ggez::graphics::Color;

pub fn get_dynamic_color(time: f32, normalized_distance: f32, in_proximity: bool) -> Color {
    let check: f32 = 0.6 + (normalized_distance * 10.0);

    // Calculate dynamic green component for yellow
    let dynamic_green = ((time.sin() * 0.25 + 0.75) * 0.5 + 0.5) as f32;

    // Define color constants with dynamic green component for YELLOW
    const RED: (f32, f32, f32) = (1.0, 0.0, 0.0); // Very Close
    let yellow = (1.0, dynamic_green, 0.0); 
    const BLUE: (f32, f32, f32) = (0.1, 0.1, 0.8); // Close
    const BLUE2: (f32, f32, f32) = (0.0, 0.0, 0.7); // Close
    // Calculate the color based on proximity
    let color = if !in_proximity {
        let factor = (7.0 - check) / 6.4;
        interpolate_colors(BLUE, yellow, factor)
    } else {
        if check < 7.0 {
            // Calculate interpolation factor for transitioning to blue
            let factor = (7.0 - check) / 6.4;
            interpolate_colors(BLUE, (0.5, dynamic_green, 0.9), factor)
        } else {
            // Calculate interpolation factor for transitioning to red
            let factor = (check - 7.0) / 3.0;
            interpolate_colors(BLUE2, (1.0, 0.3, 0.2), factor)
        }
    };

    Color::new(color.0, color.1, color.2, 1.0)
}


pub fn get_dynamic_color2(time: f32, normalized_distance: f32, in_proximity: bool) -> Color {
    let check: f32 = 0.6 + (normalized_distance * 10.0);

    // Define color constants
    const YELLOW: (f32, f32, f32) = (1.0, 1.0, 0.0); // Far
    const BLUE: (f32, f32, f32) = (0.9, 0.9, 0.0); // Close
    const RED: (f32, f32, f32) = (1.0, 0.0, 0.0); // Very Close

    // Calculate the color based on proximity
    let color = if !in_proximity {
        YELLOW // Far color
    } else {
        if check < 7.0 {
            // Calculate interpolation factor for blue
            let factor = (7.0 - check) / 6.4; // Adjust the divisor for smoother transition
            interpolate_colors(BLUE, YELLOW, factor)
        } else {
            // Calculate interpolation factor for red
            let factor = (check - 7.0) / 3.0; // Adjust the divisor for smoother transition
            interpolate_colors(RED, BLUE, factor)
        }
    };

    Color::new(color.0, color.1, color.2, 1.0)
}

// Helper function to interpolate between two colors
fn interpolate_colors(color1: (f32, f32, f32), color2: (f32, f32, f32), factor: f32) -> (f32, f32, f32) {
    let r = lerp(color1.0, color2.0, factor);
    let g = lerp(color1.1, color2.1, factor);
    let b = lerp(color1.2, color2.2, factor);
    (r, g, b)
}

pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}