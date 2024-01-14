mod player;
mod main_state;
mod collectible;
mod collectible_placement;
mod button;
mod collectible_cluster_points;
mod smoke_effect;
mod amorphous_mesh_creator;
mod proximity_and_collision_handler;
mod event_handler;
mod collidable;
mod tentacle;
mod eye;
mod heart_mesh;
mod collectibles;
mod utils;
use ggez::{conf, event, ContextBuilder};
use main_state::MainState;
const AUTHOR: &str = "badboyrenegade";
const GAME_ID: &str = "top down";
const SCREEN_WIDTH: f32 = 2000.0;
const SCREEN_HEIGHT: f32 = 2000.0;

fn main() -> ggez::GameResult {
    
    let (mut ctx, event_loop) = ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(conf::WindowSetup::default().title(GAME_ID))
        .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;
    let state = MainState::new(&mut ctx, SCREEN_WIDTH, SCREEN_HEIGHT)?;
    event::run(ctx, event_loop, state)
}
