use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, MeshBuilder, Color};

pub fn create_heart_mesh(ctx: &mut Context, size: f32, color: Color) -> GameResult<Mesh> {
    let segments = 100;
    let mut builder = MeshBuilder::new();

    let mut points: Vec<[f32; 2]> = Vec::with_capacity(segments + 1); // +1 to close the loop

    // Generate the points of a heart shape
    for i in 0..=segments { // Loop one more time to close the loop
        let t = i as f32 / segments as f32 * 2.0 * std::f32::consts::PI;
        // Parametric equations for the heart shape
        let x = 16.0 * (0.75 * t.sin()).powi(3);
        let y = 13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();

        // Scale and translate to desired size and position
        points.push([
            (size / 10.0) * x + size / 2.0, // Adjust X-offset to center the heart horizontally
            -(size / 25.0) * y + size / 2.0, // Adjust Y-offset to center the heart vertically
        ]);
    }

    // Create a filled polygon from the points
    builder.polygon(graphics::DrawMode::fill(), &points, color)?;

    // Build the heart mesh
    let heart_mesh = builder.build(ctx)?;
    Ok(heart_mesh)
}
