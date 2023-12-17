use rand::Rng; 
use crate::proximity_and_collision_handler::handle_proximity_and_collisions;
use crate::flash_effect::FlashEffect;
use crate::player::Player;
use crate::collectible::Collectible;
use crate::collidable::Collidable;
use ggez::{event, graphics, Context, GameResult};
use gilrs::Gilrs;

use crate::event_handler::EventHandler;

pub struct MainState {
    event_handler: EventHandler,
    player: Player,
    collectibles: Vec<Collectible>,
    flash_effect_pool: Vec<FlashEffect>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        //gamepad
        let gilrs = Gilrs::new().unwrap();
        //gamepad events
        let event_handler = EventHandler::new(gilrs);     
        // Initialize multiple collectibles with random positions
        let mut collectibles = Vec::new();
        let mut rng = rand::thread_rng(); // Creates a random number generator

        for i in 0..8000 {
            let x = rng.gen_range(50.0..1500.0); 
            let y = rng.gen_range(50.0..1500.0); 
            let initial_time = rng.gen_range(0.0..6.28);
        
            let id = format!("collect{}", i); // Generate an ID like "collect1", "collect2", etc.
            collectibles.push(Collectible::new(ctx, x, y, 30.0, initial_time, id)?);
        } // Corrected: This brace closes the for loop

        //Initialize multiple flash effects and put them into a pool
        let mut flash_effect_pool = Vec::new();
        for _ in 0..30 { // For example, pre-create 10 effects
            flash_effect_pool.push(FlashEffect::new_inactive()); 
        }   

        let player = Player::new();
        Ok(MainState {event_handler, player, collectibles, flash_effect_pool })
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Handle gamepad input
        self.event_handler.process_events(&mut self.player);

        // Define the proximity threshold for collectibles
        const PROXIMITY_THRESHOLD: f32 = 500.0;

        // Get a trait object for player as collidable
        let player_collidable: &dyn Collidable = &self.player;

        // Check for proximity and collisions
        let proximity_and_collisions = handle_proximity_and_collisions(
            &[player_collidable], 
            &self.collectibles.iter().collect::<Vec<&Collectible>>(),
            PROXIMITY_THRESHOLD
        );

        let mut to_remove = Vec::new();

        for (player_index, collectible_index, distance, is_collided) in proximity_and_collisions {
            if distance < PROXIMITY_THRESHOLD / 2.0 {
                // Assuming collectible_index is the index of the collectible in the collectibles vector
                if let Some(collectible) = self.collectibles.get_mut(collectible_index) {
                    collectible.set_in_proximity(true);
                   // println!("Collectible {} is within proximity threshold: {}", collectible_index, collectible.in_proximity);
                }
            } else {
                // If the collectible is not in proximity, reset its in_proximity variable
                if let Some(collectible) = self.collectibles.get_mut(collectible_index) {
                    collectible.set_in_proximity(false);
                }
            }
        
            if is_collided {
                // Mark collided collectibles for removal
                to_remove.push(collectible_index);
            }
        }
        // Remove the collectibles that collided
        for index in to_remove.iter().rev() {
            if let Some(collectible) = self.collectibles.get_mut(*index) {
                collectible.activate_flash_effect(&mut self.flash_effect_pool);
                self.collectibles.remove(*index);
            }
        }

        // Update each collectible
        for collectible in &mut self.collectibles {
            collectible.time += dt; 
        }

        // Update all flash effects
        for effect in &mut self.flash_effect_pool {
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
            collectible.draw(ctx)?;
        }

        // Draw each active flash effect
        for effect in &self.flash_effect_pool {
            if effect.is_active() {
                effect.draw(ctx)?;
            }
        }

        // Draw the player
        self.player.draw(ctx)?;

        graphics::present(ctx)
    }
}





