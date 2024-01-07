use crate::amorphous_mesh_creator;
use crate::collidable::Collidable;
use crate::eye::Eye;
use crate::smoke_effect::SmokeEffect;
use ggez::graphics::{self, Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::{Context, GameResult};
use mint::{Point2, Vector2};
use nalgebra::distance;
use noise::{NoiseFn, Perlin};
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
    pub player_direction: mint::Vector2<f32>,
    pub eye: Eye,
}

impl Collectible {
    pub fn new(
        ctx: &mut Context,
        x: f32,
        y: f32,
        size: f32,
        max_distance_threshold: f32,
        initial_time: f32,
        id: String,
        in_proximity: bool,
        distance_from_player: f32,
    ) -> GameResult<Self> {
        let eye = Eye::new(x, y, size / 10.0);
        let noise = Perlin::new();
        let normalized_distance_from_player = 0.01;
        let mesh = amorphous_mesh_creator::create_amorphous_mesh(
            ctx,
            size,
            &noise,
            initial_time,
            in_proximity,
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
            player_direction: Vector2 { x: 0.0, y: 0.0 },
            eye,
        })
    }

    pub fn update_distance(&mut self, distance: f32, max_distance_threshold: f32) {
        self.distance_from_player = distance;
        let clamped_distance = distance.clamp(10.0, max_distance_threshold);

        self.normalized_distance =
            1.0 - (clamped_distance - 10.0) / (max_distance_threshold - 10.0);
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        dt: f32,
        player_position: mint::Point2<f32>,
    ) -> GameResult<()> {
        //  println!("distance from player {}, player direction {:?}", self.normalized_distance, self.player_direction);
        self.time += dt;

        self.mesh = amorphous_mesh_creator::create_amorphous_mesh(
            ctx,
            self.size,
            &self.noise,
            self.time,
            self.in_proximity,
            self.normalized_distance,
        )?;

        // let to_player = Vector2 {
        //     x: player_position.x - self.position.x,
        //     y: player_position.y - self.position.y,
        // };

        // let distance_to_player = (to_player.x.powi(2) + to_player.y.powi(2)).sqrt();
        // let direction = if distance_to_player != 0.0 {
        //     Vector2 {
        //         x: to_player.x / distance_to_player,
        //         y: to_player.y / distance_to_player,
        //     }
        // } else {
        //     to_player // If the player is exactly at the collectible position
        // };

        // let eye_movement_scale = 10.0; // Adjust this value as needed for the effect
        // let eye_offset = Vector2 {
        //     x: direction.x * eye_movement_scale,
        //     y: direction.y * eye_movement_scale,
        // };

        // let eye_position = Point2 {
        //     x: self.position.x + eye_offset.x,
        //     y: self.position.y + eye_offset.y,
        // };
        self.eye.set_position( player_position, self.position);
        Ok(())
    }
    pub fn draw(&self, ctx: &mut Context, player_position: mint::Point2<f32>) -> GameResult<()> {
        if self.active {
            // Draw the collectible itself
            graphics::draw(
                ctx,
                &self.mesh,
                graphics::DrawParam::default()
                    .dest([self.position.x, self.position.y])
                    .scale([self.size / self.size, self.size / self.size])
                    .color(self.get_dynamic_color()),
            )?;
            self.eye.draw(ctx)?;
            // Tentacle setup
            let to_player = Vector2 {
                x: player_position.x - self.position.x,
                y: player_position.y - self.position.y,
            };

            let distance_to_player = (to_player.x.powi(2) + to_player.y.powi(2)).sqrt();
            let direction = if distance_to_player != 0.0 {
                Vector2 {
                    x: to_player.x / distance_to_player,
                    y: to_player.y / distance_to_player,
                }
            } else {
                to_player // If the player is exactly at the collectible position
            };

            let perp_direction = Vector2 {
                x: -direction.y,
                y: direction.x,
            };

            let min_tentacle_length = 2.0; // Minimum length can be the minimum for a visible line or more
            let max_tentacle_length = distance_to_player;
            // Calculate tentacle length, ensuring it's at least the minimum
            let tentacle_length =
                (self.normalized_distance * max_tentacle_length).max(min_tentacle_length);

            // Create points for the tentacle with noise
            let mut points = Vec::new();
            for i in 0..=tentacle_length as usize {
                let along = i as f32 / tentacle_length; // Normalized position along tentacle
                let noise_value =
                    self.noise.get([self.time as f64 + along as f64 * 1.2, 0.0]) as f32;
                let noise_offset = noise_value * 23.0; // Scale the noise effect

                // Calculate the vertex position with noise
                let vertex = Point2 {
                    x: self.position.x + direction.x * i as f32 + perp_direction.x * noise_offset,
                    y: self.position.y + direction.y * i as f32 + perp_direction.y * noise_offset,
                };
                points.push(vertex);
            }

            // Build the tentacle mesh from the points
            let tentacle_mesh = MeshBuilder::new()
                .polyline(DrawMode::stroke(5.0), &points, self.get_dynamic_color())?
                .build(ctx)?;

            // Draw the tentacle mesh
            graphics::draw(ctx, &tentacle_mesh, graphics::DrawParam::default())?;

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

        if !self.in_proximity {
            let g = ((self.time.sin() * 0.25 + 0.75) * 0.5 + 0.5) as f32;
            let b = 0.0 as f32;
            let r = 1.0 as f32;

            Color::new(r, g, b, 1.0)
        } else {
            //close
            if check < 7.0 {
                let g = ((self.time.sin() * 0.25 + 0.75) * 0.5 + 0.5) as f32;
                let b = 1.0 as f32;
                let r = 0.0;
                Color::new(r, g, b, 1.0)
            //very close
            } else {
                let g = 0.5;
                let b = 0.5 as f32;
                let r = 1.0 as f32;
                Color::new(r, g, b, 1.0)
            }
        }
    }

    pub fn set_in_proximity(&mut self, in_proximity: bool, distance: f32, max: f32) {
        self.in_proximity = in_proximity;
        self.distance_from_player = distance;
        self.normalized_distance = 1.0 - (distance - 50.0) / (max - 50.0);
    }
    pub fn update_eye_position(&mut self, new_position: Point2<f32>) {
        self.eye.set_position(new_position, self.position);
    }

    // Update the eye's scale, for example, if the collectible grows or shrinks
    pub fn update_eye_scale(&mut self, new_scale: f32) {
        self.eye.set_scale(new_scale);
    }

    // Change the eye's color, possibly to indicate different states or emotions
    pub fn update_eye_color(&mut self, new_color: Color) {
        self.eye.set_color(new_color);
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
