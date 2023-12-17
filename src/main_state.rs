use rand::Rng; 
use crate::collision::distance_between;
use crate::collision::check_collision;
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
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        
        //gamepad
        let gilrs = Gilrs::new().unwrap();
        //gamepad events
        let event_handler = EventHandler::new(gilrs);
       
        // Initialize multiple collectibles with random positions
        let mut collectibles = Vec::new();
        let mut rng = rand::thread_rng(); // Creates a random number generator

        for i in 0..1 {
            let x = rng.gen_range(50.0..1500.0); 
            let y = rng.gen_range(50.0..1500.0); 
            let initial_time = rng.gen_range(0.0..6.28);
        
            let id = format!("collect{}", i); // Generate an ID like "collect1", "collect2", etc.
            collectibles.push(Collectible::new(x, y, 30.0, initial_time, id));
        }

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

        // Define the proximity threshold
        const PROXIMITY_THRESHOLD: f32 = 100.0;

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
                // If the collectible is not in proximity, reset its state
                if let Some(collectible) = self.collectibles.get_mut(collectible_index) {
                    println!("In not in proximity: {}", collectible.id);
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

pub fn handle_proximity_and_collisions<T: Collidable + ?Sized, U: Collidable + ?Sized>(
    collidables1: &[&T], 
    collidables2: &[&U],
    proximity_threshold: f32
) -> Vec<(usize, usize, f32, bool)> { // Returns index1, index2, distance, and collision flag
    let mut results = Vec::new();

    for (i, collidable1) in collidables1.iter().enumerate() {
        let bbox1 = collidable1.bounding_box();

        for (j, collidable2) in collidables2.iter().enumerate() {
            let bbox2 = collidable2.bounding_box();

            let distance = distance_between(&bbox1, &bbox2);
            let is_collided = check_collision(&bbox1, &bbox2);

            // Report if within proximity threshold or if a collision has occurred
            if distance < proximity_threshold || is_collided {
                results.push((i, j, distance, is_collided));
            }
        }
    }

    results
}




// //        for index in to_remove {
//     if index < self.collectibles.len() {
//         self.collectibles[index].activate_flash_effect(&mut self.flash_effect_pool);
//         self.collectibles.remove(index);
//     }
// }