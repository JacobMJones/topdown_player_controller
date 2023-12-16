use rand::Rng; 
use crate::player::Player;
use crate::collectible::Collectible;
use ggez::{event, graphics, Context, GameResult};
use gilrs::{Event, EventType, Gilrs, Axis};
//Main state is passed context(ctx) from main.rs
pub struct MainState {
    gilrs: Gilrs,
    player: Player,
    collectibles: Vec<Collectible>,
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
        Ok(MainState { gilrs, player, collectibles })
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
        for collectible in &self.collectibles {
            let collectible_bbox = collectible.bounding_box();
            if check_collision(&player_bbox, &collectible_bbox) {
                println!("collision")
            }
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
        graphics::present(ctx)
    }
    

}