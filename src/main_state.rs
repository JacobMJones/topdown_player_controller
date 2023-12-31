use crate::collectible::Collectible;
use crate::collidable::Collidable;
use crate::event_handler::EventHandler;
use crate::player::Player;
use crate::collectible_placement;
use crate::proximity_and_collision_handler::handle_proximity_and_collisions;


use crate::smoke_effect::SmokeEffect;
use ggez::{event, graphics, Context, GameResult};
use gilrs::Gilrs;
use rand::Rng;
const COLLECTIBLE_SIZE: f32 = 140.0;
const COLLECTIBLE_COUNT: i32 = 800;
const CLUSTER_SIZE: f32 = 200.0;
const PARTICLES_IN_SMOKE: i32 = 10;
const PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD: f32 = 800.0;
pub struct MainState {
    event_handler: EventHandler,
    player: Player,
    collectibles: Vec<Collectible>,
    smoke_effect_pool: Vec<SmokeEffect>,
}

impl MainState {
    pub fn new(ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<MainState> {

        //gamepad
        let gilrs = Gilrs::new().unwrap();
        //gamepad events
        let event_handler = EventHandler::new(gilrs);

        let collectibles = collectible_placement::generate_collectibles(
            ctx,
            screen_width,
            screen_height,
            COLLECTIBLE_COUNT,
            CLUSTER_SIZE,
            PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD,
            COLLECTIBLE_SIZE,
        )?;


        //Initialize multiple smoke effects and put them into a pool
        let mut smoke_effect_pool = Vec::new();
        for _ in 0..PARTICLES_IN_SMOKE {
            smoke_effect_pool.push(SmokeEffect::new_inactive());
        }

        let player = Player::new();
        Ok(MainState {
            event_handler,
            player,
            collectibles,
            smoke_effect_pool,
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Handle gamepad input
        self.event_handler.process_events(&mut self.player);

        // Get a trait object for player as collidable
        let player_collidable: &dyn Collidable = &self.player;

        // Check for proximity and collisions
        let proximity_and_collisions = handle_proximity_and_collisions(
            &[player_collidable],
            &self.collectibles.iter().collect::<Vec<&Collectible>>(),
            PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD,
        );

        //set which collectibles should be removed due to collision
        let mut to_remove = Vec::new();

        for (_player_index, collectible_index, distance, is_collided) in proximity_and_collisions {
            if distance < PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD / 2.0 {
                // Assuming collectible_index is the index of the collectible in the collectibles vector
                if let Some(collectible) = self.collectibles.get_mut(collectible_index) {
                    collectible.update_distance(distance, PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD);
                    collectible.set_in_proximity(true, distance, PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD);
                }
            } else {
                // If the collectible is not in proximity, reset its in_proximity variable
                if let Some(collectible) = self.collectibles.get_mut(collectible_index) {
               //     collectible.update_distance(distance, PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD);
                    collectible.set_in_proximity(false, distance, PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD);
                }
            }

            if is_collided {
                // Mark collided collectibles for removal
                to_remove.push(collectible_index);
            }
        }
        // Remove the collectibles that collided with the player
        for index in to_remove.iter().rev() {
            if let Some(collectible) = self.collectibles.get_mut(*index) {
                collectible.activate_smoke_effect(&mut self.smoke_effect_pool);
                self.collectibles.remove(*index);
            }
        }
        // Update each collectible
        for collectible in &mut self.collectibles {
            collectible.player_direction = self.player.direction;
            collectible.update(ctx, dt, self.player.position);
        }
        // Update all smoke effects
        for effect in &mut self.smoke_effect_pool {
            effect.update(dt);
        }
        // Update the player
        self.player.update(dt);

        Ok(())
    }


    

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));

        // Draw each collectible
        for collectible in &self.collectibles {
            collectible.draw(ctx, self.player.position)?;
        }
        // Draw each active smoke effect
        for effect in &self.smoke_effect_pool {
            if effect.is_active() {
                effect.draw(ctx)?;
            }
        }
        // Draw the player
        self.player.draw(ctx)?;

        graphics::present(ctx)
    }
}
