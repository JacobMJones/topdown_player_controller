use crate::amorphous_mesh_creator;
use crate::collidable::Collidable;
use crate::eye::Eye;
use crate::heart_mesh;
use crate::smoke_effect::SmokeEffect;
use crate::tentacle::Tentacle;
use crate::utils::get_dynamic_color;
use ggez::graphics::{self, Color, Mesh, Rect};
use ggez::{Context, GameResult};
use mint::{Point2, Vector2};

use noise::Perlin;
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
    pub heart_mesh: Mesh,
    noise: Perlin,
    pub player_direction: mint::Vector2<f32>,
    pub eye: Eye,
    pub tentacle: Tentacle,
    pub color: Color,
    pub max_distance_threshold: f32,
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
        let heart_mesh = heart_mesh::create_heart_mesh(ctx, size/2.0, Color::new(1.0, 0.0, 0.0, 1.0))?;

        let noise = Perlin::new();
        let color: Color = Color::new(1.0, 0.0, 0.0, 1.0);
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
            distance_from_player: 1000.0,
            normalized_distance: normalized_distance_from_player,
            mesh,
            heart_mesh,
            noise,
            player_direction: Vector2 { x: 0.0, y: 0.0 },
            eye,
            tentacle: Tentacle::new(Point2 { x, y }, 5.0, Color::new(1.0, 0.5, 0.5, 0.0), 2.5),
            color,
            max_distance_threshold,
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
        self.time += dt;

        self.mesh = amorphous_mesh_creator::create_amorphous_mesh(
            ctx,
            self.size,
            &self.noise,
            self.time,
            self.in_proximity,
            self.normalized_distance,
        )?;

        self.color = get_dynamic_color(self.time, self.normalized_distance, self.in_proximity);

        self.tentacle.update(
            ctx,
            player_position,
            250.0,
            self.normalized_distance,
            self.time,
            self.color,
            self.in_proximity,
            self.max_distance_threshold,
        )?;

         self.eye.update(player_position, self.position, self.distance_from_player, self.in_proximity);

        Ok(())
    }
    pub fn draw(&self, ctx: &mut Context, player_position: mint::Point2<f32>) -> GameResult<()> {
        if self.active {
            if self.distance_from_player < 200.0 {
                graphics::draw(
                    ctx,
                    &self.heart_mesh,
                    graphics::DrawParam::default()
                        .dest([self.position.x, self.position.y])
                        .scale([self.size / self.size, self.size / self.size])
                        .color(self.color),
                )?;
            } else {
                graphics::draw(
                    ctx,
                    &self.mesh,
                    graphics::DrawParam::default()
                        .dest([self.position.x, self.position.y])
                        .scale([self.size / self.size, self.size / self.size])
                        .color(self.color),
                )?;
            }

             self.eye.draw(ctx)?;
            self.tentacle.draw(ctx)?;
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

    pub fn set_in_proximity(&mut self, in_proximity: bool, distance: f32, max: f32) {
        self.in_proximity = in_proximity;
        self.distance_from_player = distance;
        self.normalized_distance = 1.0 - (distance - 50.0) / (max - 50.0);
    }
}

impl Collidable for Collectible {
    fn bounding_box(&self) -> Rect {
        self.bounding_box()
    }
}
