use crate::collidable::Collidable;
use crate::smoke_effect::SmokeEffect;
use ggez::graphics::{self, Color, Mesh, DrawMode, MeshBuilder, Rect};
use ggez::{Context, GameResult};
use mint::Point2;
use noise::{NoiseFn, Perlin};

pub struct Collectible {
    pub position: Point2<f32>,
    pub size: f32,
    pub active: bool,
    pub radius: f32,
    pub time: f32,
    pub id: String,
    pub in_proximity: bool,
    mesh: Mesh,
    noise: Perlin,
}

impl Collectible {
    pub fn new(ctx: &mut Context, x: f32, y: f32, size: f32, initial_time: f32, id: String) -> GameResult<Self> {
        let noise = Perlin::new();
        let mesh = create_amorphous_mesh(ctx, size, &noise, initial_time)?;
        Ok(Collectible {
            position: Point2 { x, y },
            size,
            active: true,
            radius: size / 2.0,
            time: initial_time,
            id,
            in_proximity: false,
            mesh,
            noise
        })
    }

    pub fn update(&mut self, ctx: &mut Context, dt: f32) -> GameResult<()> {
        self.time += dt;
        self.mesh = create_amorphous_mesh(ctx, self.size, &self.noise, self.time)?;
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.active {
            let size = self.get_pulsating_size();
            let color = self.get_dynamic_color();

            let scale_x = size / self.size;
            let scale_y = size / self.size;

            graphics::draw(
                ctx,
                &self.mesh,
                graphics::DrawParam::default()
                    .dest([self.position.x, self.position.y])
                    .scale([scale_x, scale_y])
                    .color(color),
            )?;
        }
        Ok(())
    }

    pub fn bounding_box(&self) -> Rect {
        Rect::new(
            self.position.x - self.size / 2.0,
            self.position.y - self.size / 2.0,
            self.size,
            self.size,
        )
    }

    pub fn activate_smoke_effect(&self, smoke_effect_pool: &mut Vec<SmokeEffect>) {
        let base_position = Point2 {
            x: self.position.x + self.size / 2.0,
            y: self.position.y + self.size / 2.0,
        };
        for _ in 0..5 {
            if let Some(inactive_effect) = smoke_effect_pool.iter_mut().find(|e| !e.is_active()) {
                inactive_effect.activate(base_position);
            }
        }
    }

    fn get_pulsating_size(&self) -> f32 {
        let pulsation_factor = 0.9;
        let min_size = 10.0;
        let max_size = self.size;
        let mut pulsating_size = self.size + pulsation_factor * self.time.sin();

        if pulsating_size < min_size {
            pulsating_size = min_size + (min_size - pulsating_size);
        } else if pulsating_size > max_size {
            pulsating_size = max_size - (pulsating_size - max_size);
        }

        pulsating_size
    }

    fn get_dynamic_color(&self) -> Color {
        if !self.in_proximity {
            let r = (self.time.sin() * 0.5 + 0.5) as f32;
            let g = ((self.time + 2.0).sin() * 0.5 + 0.5) as f32;
            let b = ((self.time + 4.0).sin() * 0.5 + 0.5) as f32;
            Color::new(r, g, b, 1.0)
        } else {
            Color::new(1.0, 1.0, 1.0, 0.1)
        }
    }

    pub fn set_in_proximity(&mut self, in_proximity: bool) {
        self.in_proximity = in_proximity;
    }
}

impl Collidable for Collectible {
    fn bounding_box(&self) -> Rect {
        self.bounding_box()
    }
}

fn create_circle_mesh(ctx: &mut Context, size: f32) -> GameResult<Mesh> {
    graphics::Mesh::new_circle(
        ctx,
        DrawMode::fill(),
        Point2 { x: 0.0, y: 0.0 },
        size / 2.0,
        0.1, // Smoothness of the circle
        Color::WHITE,
    )
}
fn create_amorphous_mesh(ctx: &mut Context, size: f32, noise: &Perlin, time: f32) -> GameResult<Mesh> {
    let mut builder = MeshBuilder::new();
    let num_points = 50;
    let angle_step = 2.0 * std::f32::consts::PI / num_points as f32;

    let noise_scale = 0.5; // How "zoomed in" you are on the noise
    let time_scale = 0.1; // How fast the noise changes over time
    let max_allowed_variation: f32 = size * 0.05; // Maximum change allowed between points

    let mut prev_radius = size / 2.0; // Start with the base radius
    let mut points = Vec::new();

    let mut angle: f32 = 0.0; // Explicitly specify the type as f32

    for _ in 0..num_points {
        // Calculate the noise sample coordinates
        let noise_x = (angle.cos() * noise_scale + time * time_scale) as f64;
        let noise_y = (angle.sin() * noise_scale + time * time_scale) as f64;

        // Sample the noise function to get the radius variation
        let radius_variation: f32 = noise.get([noise_x, noise_y]) as f32;
        let base_radius = size / 2.0;
        let mut new_radius = base_radius + radius_variation * size * 0.2;

        // Enforce the max allowed variation for a smoother shape
        let radius_change = new_radius - prev_radius;
        if radius_change.abs() > max_allowed_variation {
            new_radius = if radius_change > 0.0 {
                prev_radius + max_allowed_variation
            } else {
                prev_radius - max_allowed_variation
            };
        }

        // Calculate the position of the current point
        let x = new_radius * angle.cos();
        let y = new_radius * angle.sin();

        // Add this point to the points vector
        points.push(Point2 { x, y });

        // Update prev_radius for the next iteration
        prev_radius = new_radius;

        // Increment the angle for the next point
        angle += angle_step;
    }
    // Connect the last point to the first to close the shape
    builder.polygon(
        DrawMode::fill(),
        &points,
        Color::from_rgb(255, 255, 255) // Use a white color for the filled polygon
    )?;

    builder.build(ctx)
}