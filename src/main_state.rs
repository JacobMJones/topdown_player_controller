use rand::Rng; 
use crate::proximity_and_collision_handler::handle_proximity_and_collisions;
use crate::smoke_effect::SmokeEffect;
use crate::player::Player;
use crate::collectible::Collectible;
use crate::collidable::Collidable;
use ggez::{event, graphics, Context, GameResult};
use gilrs::Gilrs;
use crate::event_handler::EventHandler;
const SCREEN_WIDTH: f32 = 1000.0;
const SCREEN_HEIGHT: f32 = 1000.0;

const CLUSTER_POINTS: &[(f32, f32); 5] = &[
    (SCREEN_WIDTH * 0.15, SCREEN_HEIGHT * 0.16),   // Example near the top-left
    (SCREEN_WIDTH * 0.8, SCREEN_HEIGHT * 0.8),     // Example near the bottom-right
    (SCREEN_WIDTH * 0.01, SCREEN_HEIGHT * 0.8),    // Example near the bottom-left
    (SCREEN_WIDTH * 0.2, SCREEN_HEIGHT * 0.01),    // Example near the top-center
    (SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.8),     // Example near the middle-right
];
const COLLECTIBLE_COUNT: i32 = 300;
const CLUSTER_SIZE: f32 = 100.0;

pub struct MainState {
    event_handler: EventHandler,
    player: Player,
    collectibles: Vec<Collectible>,
    smoke_effect_pool: Vec<SmokeEffect>,
    screen_width: f32,
    screen_height: f32,
}

impl MainState {
    pub fn new(ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<MainState> {

        let cluster_points: &[(f32, f32); 5] = &[
            (screen_width * 0.15, screen_height * 0.16),  // Adjust these as necessary
            (screen_width * 0.8, screen_height * 0.8),
            (screen_width * 0.01, screen_height * 0.8),
            (screen_width * 0.2, screen_height * 0.01),
            (screen_width * 0.5, screen_height * 0.8),
        ];
        //gamepad
        let gilrs = Gilrs::new().unwrap();
        //gamepad events
        let event_handler = EventHandler::new(gilrs);   

        // Initialize multiple collectibles with random positions
        let mut collectibles = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..COLLECTIBLE_COUNT{
            // Choose a random cluster point
            let (center_x, center_y) = CLUSTER_POINTS[rng.gen_range(0..CLUSTER_POINTS.len())];

            // Generate positions near the cluster point
            let x = rng.gen_range(center_x as f32 - CLUSTER_SIZE..=center_x as f32 + CLUSTER_SIZE).clamp(50.0, 2000.0);
            let y = rng.gen_range(center_y as f32 - CLUSTER_SIZE..=center_y as f32 + CLUSTER_SIZE).clamp(50.0, 2000.0);
            
            //adds randomness to shapeshifting startpoint
            let initial_time = rng.gen_range(0.0..6.28);

            let id = format!("collect{}", i);
            collectibles.push(Collectible::new(ctx, x, y, 70.0, initial_time, id, false)?);
        }

        //Initialize multiple smoke effects and put them into a pool
        let mut smoke_effect_pool = Vec::new();
        for _ in 0..30 { // For example, pre-create 10 effects
            smoke_effect_pool.push(SmokeEffect::new_inactive()); 
        }   

        let player = Player::new();
        Ok(MainState {
            event_handler, 
            player, 
            collectibles, 
            smoke_effect_pool,            
            screen_width,
            screen_height, })
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
            collectible.update(ctx, dt);
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





