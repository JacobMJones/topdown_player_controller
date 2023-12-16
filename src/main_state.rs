use rand::Rng; 
use crate::flash_effect::FlashEffect;
use crate::player::Player;
use crate::collectible::Collectible;
use ggez::{event, graphics, Context, GameResult};
use ggez::graphics::Color;
use gilrs::{Event, EventType, Gilrs, Axis};
use mint::Point2;
//Main state is passed context(ctx) from main.rs
pub struct MainState {
    gilrs: Gilrs,
    player: Player,
    collectibles: Vec<Collectible>,
    flash_effect_pool: Vec<FlashEffect>,
}
fn check_collision(rect1: &graphics::Rect, rect2: &graphics::Rect) -> bool {
    rect1.x < rect2.x + rect2.w &&
    rect1.x + rect1.w > rect2.x &&
    rect1.y < rect2.y + rect2.h &&
    rect1.y + rect1.h > rect2.y
}
impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let gilrs = Gilrs::new().unwrap();
        let player = Player::new();
        // Initialize multiple collectibles with random positions
        let mut collectibles = Vec::new();
        let mut rng = rand::thread_rng(); // Create a random number generator
        for _ in 0..20 {
            // Generate random x and y within the range -1000 to 1000
            let x = rng.gen_range(50.0..1200.0); 
            let y = rng.gen_range(50.0..1000.0); 
            collectibles.push(Collectible::new(x, y,50.0)); // Adjust the third parameter as needed
        }
        let mut flash_effect_pool = Vec::new();
        for _ in 0..10 { // For example, pre-create 10 effects
            flash_effect_pool.push(FlashEffect::new_inactive()); // You need to create this method
        }   
        Ok(MainState { gilrs, player, collectibles, flash_effect_pool })
    }
}
impl event::EventHandler<ggez::GameError> for MainState {
    //always fires
    fn update(&mut self, ctx: &mut Context) -> GameResult {
      

        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // fires when a new event is available (from gilrs - controller)
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            if let EventType::AxisChanged(axis, value, ..) = event {
                match axis {
                    Axis::LeftStickX => self.player.axis_left.0 = value,
                    Axis::LeftStickY => self.player.axis_left.1 = -value,
                    Axis::RightStickX => self.player.axis_right.0 = value,
                    Axis::RightStickY => self.player.axis_right.1 = -value,
                    _ => (),
                }
            }
        }
        let player_bbox = self.player.bounding_box();
        let mut to_remove = Vec::new();
        for (index, collectible) in self.collectibles.iter().enumerate() {
            let collectible_bbox = collectible.bounding_box();
            if check_collision(&player_bbox, &collectible_bbox) {

                // Additional logic for collision
                if let Some(inactive_effect) = self.flash_effect_pool.iter_mut().find(|e| !e.is_active()) {
        
                    
                    // Create a new Point2, adjust position
                    let adjusted_position = Point2 { x: collectible.position.x + collectible.size/2.0, y: collectible.position.y + collectible.size/2.0 };
                    
                    // Pass the adjusted_position to the activate method
                    inactive_effect.activate(
                        adjusted_position,
                        Color::new(1.0, 0.0, 0.0, 1.0), // Red color
                        0.5, // Duration
                    );
                }
                println!("Collided with collectible at position: ({}, {})", collectible.position.x, collectible.position.y);
                to_remove.push(index);
            }
        }

        // Remove collided collectibles
        for index in to_remove.into_iter().rev() {
            self.collectibles.remove(index);
        }
        for effect in &mut self.flash_effect_pool {
            effect.update(dt);
        }
        self.player.update(dt);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
        self.player.draw(ctx)?;
        for collectible in &self.collectibles {
            collectible.draw(ctx)?;
        }
        // Draw active flash effects
        for effect in &self.flash_effect_pool {
            if effect.is_active() {
                effect.draw(ctx)?;
            }
        }
        graphics::present(ctx)
    }
}