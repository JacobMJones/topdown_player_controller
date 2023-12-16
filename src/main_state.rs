use rand::Rng; 
use crate::collision;
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
        for _ in 0..1500 {
            let x = rng.gen_range(50.0..1500.0); 
            let y = rng.gen_range(50.0..1500.0); 
            let initial_time = rng.gen_range(0.0..6.28); // Random time value, for example

            collectibles.push(Collectible::new(x, y,30.0, initial_time)); 
        }

        //Initialize multiple flash effects and put them into a pool
        let mut flash_effect_pool = Vec::new();
        for _ in 0..10 { // For example, pre-create 10 effects
            flash_effect_pool.push(FlashEffect::new_inactive()); // You need to create this method
        }   

        let player = Player::new();
        Ok(MainState {event_handler, player, collectibles, flash_effect_pool })
    }

}

impl event::EventHandler<ggez::GameError> for MainState {
    // Game loop's update method
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Handle gamepad input
        self.event_handler.process_events(&mut self.player);

        // Create a trait object for player as collidable
        let player_collidable: &dyn Collidable = &self.player;


        //Collectible Collisions With Player
        let collision_pairs = collision::handle_collisions(&[player_collidable], &self.collectibles.iter().collect::<Vec<&Collectible>>());

        // Collect indices of collectibles to be removed
        let mut to_remove: Vec<usize> = collision_pairs.iter().map(|&(_, collectible_index)| collectible_index).collect();
        // Sort and deduplicate
        to_remove.sort_unstable_by(|a, b| b.cmp(a)); 
        to_remove.dedup(); 
        
        // Remove the collectibles
        for index in to_remove {
            if index < self.collectibles.len() {
                self.collectibles[index].activate_flash_effect(&mut self.flash_effect_pool);
                self.collectibles.remove(index);
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

    // Game loop's draw method
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
       
        // Draw the player
        self.player.draw(ctx)?;

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

        graphics::present(ctx)
    }
}