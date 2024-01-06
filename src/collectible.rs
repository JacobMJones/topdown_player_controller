use crate::collidable::Collidable;
use crate::smoke_effect::SmokeEffect;
use ggez::graphics::{self, Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::{Context, GameResult};
use mint::Point2;
use noise::{NoiseFn, Perlin};
use crate::amorphous_mesh_creator;
pub struct Collectible {
    pub position: Point2<f32>,
    pub size: f32,
    pub active: bool,
    pub radius: f32,
    pub time: f32,
    pub id: String,
    pub in_proximity: bool,
    pub distance_from_player: f32,
    pub normalized_distance: f32, 
    mesh: Mesh,
    noise: Perlin,
}

impl Collectible {
    pub fn new(
        ctx: &mut Context,
        x: f32,
        y: f32,
        size: f32,
        initial_time: f32,
        id: String,
        in_proximity: bool,
        distance_from_player: f32,
    ) -> GameResult<Self> {
        let noise = Perlin::new();
        let normalized_distance_from_player = 1.0 - (distance_from_player - 50.0) / (1000.0 - 50.0);
        let mesh = amorphous_mesh_creator::create_amorphous_mesh(
            ctx,
            size,
            &noise,
            initial_time,
            in_proximity,
            distance_from_player,
            normalized_distance_from_player, 
        )?;
        Ok(Collectible {
            position: Point2 { x, y },
            size,
            active: true,
            radius: size / 2.0,
            time: initial_time,
            id,
            in_proximity: false,
            distance_from_player,
            normalized_distance: normalized_distance_from_player, 
            mesh,
            noise,
        })
    }

    pub fn update(&mut self, ctx: &mut Context, dt: f32, distance: f32) -> GameResult<()> {
        self.time += dt;
        self.distance_from_player = distance;
        self.normalized_distance = 1.0 - (distance - 50.0) / (1000.0 - 50.0);
   
        self.mesh = amorphous_mesh_creator::create_amorphous_mesh(
            ctx,
            self.size,
            &self.noise,
            self.time,
            self.in_proximity,
            self.distance_from_player,
            self.normalized_distance, 
        )?;
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.active {
            graphics::draw(
                ctx,
                &self.mesh,
                graphics::DrawParam::default()
                    .dest([self.position.x, self.position.y])
                    .scale([self.size / self.size, self.size / self.size]) 
                    .color(self.get_dynamic_color()), 
            )?;

            //draw eye
            let eased_distance = smootherstep(0.0, 1.0, self.normalized_distance);
            let circle_radius = (self.size * eased_distance) / 8.0; 
            let circle_color = Color::new(1.0, 1.0, 1.0, 0.8); 
            let circle_mesh = MeshBuilder::new()
                .circle(
                    DrawMode::fill(),
                    [0.0, 0.0], // Center of the circle, it will be positioned correctly by the .dest field
                    circle_radius,
                    1.0, 
                    circle_color,
                )?
                .build(ctx)?;

            // Draw the circle mesh (eye)
            graphics::draw(
                ctx,
                &circle_mesh,
                graphics::DrawParam::default().dest([self.position.x, self.position.y]), // Position it at the collectible's center
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

    fn get_dynamic_color(&self) -> Color {
        let check: f32 = 0.6 + (self.normalized_distance * 10.0);
        // println!("check {}", check);
        if !self.in_proximity {
            let g = ((self.time.sin() * 0.25 + 0.75) * 0.5 + 0.5) as f32;
            let b = 0.0 as f32;
            let r = 1.0 as f32;

            Color::new(r, g, b, 1.0)
        } else {
            if check > 10.0 {
                let g = ((self.time.sin() * 0.25 + 0.75) * 0.5 + 0.5) as f32;
                let b = 1.0 as f32;
                let r = self.normalized_distance as f32;
                Color::new(r, g, 1.0, 1.0)
            } else {
                let g = self.normalized_distance as f32;
                let b = 1.0 as f32;
                let r = 0.0 as f32;
                Color::new(r, g, 1.0, 1.0)
            }
        }
    }

    pub fn set_in_proximity(&mut self, in_proximity: bool, distance: f32, max: f32 ) {
        self.in_proximity = in_proximity;
        self.distance_from_player = distance; 
        self.normalized_distance = 1.0 - (distance - 50.0) / (1000.0 - 50.0);
    }
}

impl Collidable for Collectible {
    fn bounding_box(&self) -> Rect {
        self.bounding_box()
    }
}

fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}
