use crate::collectible::Collectible;
use crate::collidable::Collidable;
use crate::collectible_placement;
use crate::smoke_effect::SmokeEffect;
use ggez::{event, graphics, Context, GameResult};
use mint::{Point2, Vector2};
use noise::utils::PlaneMapBuilder;
use crate::player::Player;
use rand::Rng;
use uuid::Uuid;
use crate::proximity_and_collision_handler::handle_proximity_and_collisions;

pub struct Collectibles {
    pub items: Vec<Collectible>,
    collectible_count: i32,
    collectible_size: f32,
    cluster_size: f32,
    max_distance_threshold: f32,
    screen_width: f32,
    screen_height: f32,
}

impl Collectibles {
    pub fn new(
        ctx: &mut Context, 
        screen_width: f32, 
        screen_height: f32, 
        collectible_count: i32, 
        max_distance_threshold: f32, 
        cluster_size: f32, 
        collectible_size: f32
    ) -> GameResult<Collectibles> {
        let items = Collectibles::setup_collectibles(
            ctx,
            screen_width,
            screen_height,
            collectible_count,
            cluster_size,
            max_distance_threshold,
            collectible_size,
        )?;

        Ok(Collectibles {
            items,
            collectible_count,
            collectible_size,
            cluster_size,
            max_distance_threshold,
            screen_width,
            screen_height,
        })
    }

    pub fn reset_collectibles(&mut self, ctx: &mut Context) {
        self.items = Collectibles::setup_collectibles(
            ctx,
            self.screen_width,
            self.screen_height,
            self.collectible_count,
            self.cluster_size,
            self.max_distance_threshold,
            self.collectible_size,
        ).expect("Failed to generate collectibles");
    }

    fn setup_collectibles(
        ctx: &mut Context,
        screen_width: f32,
        screen_height: f32,
        collectible_count: i32,
        cluster_size: f32,
        max_distance_threshold: f32,
        collectible_size: f32,
    ) -> GameResult<Vec<Collectible>> {
        collectible_placement::generate_collectibles(
            ctx,
            screen_width,
            screen_height,
            collectible_count,
            cluster_size,
            max_distance_threshold,
            collectible_size,
        )
    }

    pub fn update(&mut self, ctx: &mut Context, dt: f32, player_position: Point2<f32>, player: &Player) {
        let player_collidable: &dyn Collidable = player;

        let proximity_and_collisions = handle_proximity_and_collisions(
            &[player_collidable],
            &self.items.iter().collect::<Vec<&Collectible>>(),
            800.0,
        );

        let mut to_remove = Vec::new();

        for (_player_index, collectible_index, distance, is_collided) in proximity_and_collisions {
            self.handle_collectible_proximity(collectible_index, distance);
            if is_collided {
                self.handle_collectible_collision(collectible_index, &mut to_remove);
            }
        }

        for index in to_remove.iter().rev() {
            if let Some(collectible) = self.items.get_mut(*index) {
                // collectible.activate_smoke_effect(&mut self.smoke_effect_pool);
                self.items.remove(*index);
            }
        }

        for collectible in &mut self.items {
            collectible.player_direction = player.direction;
            collectible.update(ctx, dt, player_position);
        }

        // Update all smoke effects in the pool
        // for effect in smoke_effect_pool {
        //     effect.update(dt);
        // }
    }

    pub fn handle_collectible_proximity(&mut self, collectible_index: usize, distance: f32) {
        if let Some(collectible) = self.items.get_mut(collectible_index) {
            let in_proximity = distance < 1000.0 / 2.0;
            collectible.set_in_proximity(
                in_proximity,
                distance,
                1000.0,
            );
        }
    }

    fn handle_collectible_collision(
        &mut self,
        collectible_index: usize,
        to_remove: &mut Vec<usize>,
    ) {
        to_remove.push(collectible_index);
    }

    pub fn draw(&self, ctx: &mut Context, player_position: mint::Point2<f32>) -> GameResult<()> {
        for collectible in &self.items {
            collectible.draw(ctx, player_position)?;
        }
        Ok(())
    }

    // Additional methods can be added as needed
}