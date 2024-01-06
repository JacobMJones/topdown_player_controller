
use ggez::graphics::{self, Color, DrawMode, Mesh, MeshBuilder};
use ggez::{Context, GameResult};
use mint::Point2;
use noise::{NoiseFn, Perlin};

pub fn create_amorphous_mesh(
    ctx: &mut Context,
    size: f32,
    noise: &Perlin,
    time: f32,
    in_proximity: bool,
    normalized_distance_from_player: f32,
) -> GameResult<Mesh> {
    let mut builder = MeshBuilder::new();

    let num_points = 20;
    let angle_step = (2.0 * std::f32::consts::PI) / num_points as f32;
    let (noise_scale_start, noise_scale_end) = (0.3, 0.6);  // not in proximity to in proximity
    let (time_scale_start, time_scale_end) = (0.2, 0.6); 

  let noise_scale = noise_scale_start + (noise_scale_end - noise_scale_start) * normalized_distance_from_player;
    let time_scale = time_scale_start + (time_scale_end - time_scale_start) * normalized_distance_from_player;


    let mut points = Vec::new();

    let base_radius = size / 2.0;
    let min_radius = base_radius * 0.4;
    let noise_amplitude = base_radius * normalized_distance_from_player;

    // First pass: calculate points for the blob
    for i in 0..num_points {
        let angle = i as f32 * angle_step;
        let noise_x = (angle.cos() * noise_scale + time * time_scale) as f64;
        let noise_y = (angle.sin() * noise_scale + time * time_scale) as f64;
        let noise_value = noise.get([noise_x, noise_y]) as f32;
        let noise_offset = noise_value * noise_amplitude;
        let radius = (base_radius + noise_offset).max(min_radius);
        let x = radius * angle.cos();
        let y = radius * angle.sin();

        points.push(Point2 { x, y });
    }

    // Second pass: smooth the points
    let smoothed_points = smooth_points(&points);

    // Build the polygon with smoothed points
    builder.polygon(
        DrawMode::fill(),
        &smoothed_points,
        Color::from_rgb(255, 255, 255),
    )?;

    builder.build(ctx)
}

fn smooth_points(points: &[Point2<f32>]) -> Vec<Point2<f32>> {
    let len = points.len();
    let mut smoothed_points = Vec::with_capacity(len);

    for i in 0..len {
        let prev = if i == 0 {
            points[len - 1]
        } else {
            points[i - 1]
        };
        let next = if i == len - 1 {
            points[0]
        } else {
            points[i + 1]
        };
        let current = points[i];

        let avg_x = (prev.x + current.x + next.x) / 3.0;
        let avg_y = (prev.y + current.y + next.y) / 3.0;

        smoothed_points.push(Point2 { x: avg_x, y: avg_y });
    }

    smoothed_points
}