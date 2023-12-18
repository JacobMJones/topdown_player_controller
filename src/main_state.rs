use rand::Rng; 
use crate::proximity_and_collision_handler::handle_proximity_and_collisions;
use crate::smoke_effect::SmokeEffect;
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
    smoke_effect_pool: Vec<SmokeEffect>,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        //gamepad
        let gilrs = Gilrs::new().unwrap();
        //gamepad events
        let event_handler = EventHandler::new(gilrs);     
        // Initialize multiple collectibles with random positions
        let cluster_points = vec![
            (1000.0, 1000.0), // Example cluster center
            (500.0, 2000.0),  // Another cluster center
            // Add more cluster centers as needed
        ];

        let mut collectibles = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..10000 {
            // Choose a random cluster point
            let (center_x, center_y) = cluster_points[rng.gen_range(0..cluster_points.len())];

            // Generate positions near the cluster point
            let x = rng.gen_range(center_x - 1000.0..center_x + 1000.0); // Adjust range for clustering
            let y = rng.gen_range(center_y - 1000.0..center_y + 1000.0); // Adjust range for clustering

            let initial_time = rng.gen_range(0.0..6.28);
            let id = format!("collect{}", i);
            collectibles.push(Collectible::new(ctx, x, y, 50.0, initial_time, id)?);
        }
        //Initialize multiple smoke effects and put them into a pool
        let mut smoke_effect_pool = Vec::new();
        for _ in 0..30 { // For example, pre-create 10 effects
            smoke_effect_pool.push(SmokeEffect::new_inactive()); 
        }   

        let player = Player::new();
        Ok(MainState {event_handler, player, collectibles, smoke_effect_pool })
    }
}


impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Handle gamepad input
        self.event_handler.process_events(&mut self.player);

        // Define the proximity threshold for collectibles
        const PROXIMITY_THRESHOLD: f32 = 300.0;

        // Get a trait object for player as collidable
        let player_collidable: &dyn Collidable = &self.player;

        // Check for proximity and collisions
        let proximity_and_collisions = handle_proximity_and_collisions(
            &[player_collidable], 
            &self.collectibles.iter().collect::<Vec<&Collectible>>(),
            PROXIMITY_THRESHOLD
        );

        let mut to_remove = Vec::new();

        for (_player_index, collectible_index, distance, is_collided) in proximity_and_collisions {
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
                collectible.activate_smoke_effect(&mut self.smoke_effect_pool);
                self.collectibles.remove(*index);
            }
        }

        // Update each collectible
        for collectible in &mut self.collectibles {
            collectible.update(dt); 
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
            collectible.draw(ctx)?;
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





