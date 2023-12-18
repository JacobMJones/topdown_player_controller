use crate::collidable::Collidable;
use crate::smoke_effect::SmokeEffect;
use ggez::graphics::{self, Color, Mesh, DrawMode, Rect};
use ggez::{Context, GameResult};
use mint::Point2;
use rand::Rng;

pub struct Collectible {
    pub position: Point2<f32>,
    pub size: f32,
    pub active: bool,
    pub radius: f32,
    pub time: f32,
    pub id: String,
    pub in_proximity: bool,
    mesh: Mesh,
}

impl Collectible {
    pub fn new(ctx: &mut Context, x: f32, y: f32, size: f32, initial_time: f32, id: String) -> GameResult<Self> {
        let mesh = create_circle_mesh(ctx, size)?;
        Ok(Collectible {
            position: Point2 { x, y },
            size,
            active: true,
            radius: size / 2.0,
            time: initial_time,
            id,
            in_proximity: false,
            mesh,
        })
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
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
        let offset_range = 10.0; // Define a range for the random offset
        let color = Color::new(1.0, 1.0, 1.0, 1.0); // White color
        let duration = 0.4; // Duration for each smoke effect

        for _ in 0..4 {
            if let Some(inactive_effect) = smoke_effect_pool.iter_mut().find(|e| !e.is_active()) {
                inactive_effect.activate(base_position, offset_range, color, duration);
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
