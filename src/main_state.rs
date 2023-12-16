use rand::Rng; 
use crate::collision;
use crate::flash_effect::FlashEffect;
use crate::player::Player;
use crate::collectible::Collectible;
use ggez::{event, graphics, Context, GameResult};
use ggez::graphics::Color;
use gilrs::Gilrs;
use mint::Point2;
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
            collectibles.push(Collectible::new(x, y,30.0)); 
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
    //always fires
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        self.event_handler.process_events(&mut self.player);

        let to_remove = collision::handle_collisions(&mut self.player, &mut self.collectibles);

        for index in to_remove.into_iter().rev() {
            self.collectibles[index].activate_flash_effect(&mut self.flash_effect_pool);
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