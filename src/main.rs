mod player;
mod main_state;
mod collectible;
mod flash_effect;
mod proximity_and_collision_handler;
mod event_handler;
mod collidable;
use ggez::{conf, event, ContextBuilder};
use main_state::MainState;


fn main() -> ggez::GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("top_down_shooter", "author")
        .window_setup(conf::WindowSetup::default().title("Top Down Shooter"))
        .window_mode(conf::WindowMode::default().dimensions(1400.0, 1400.0))
        .build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
