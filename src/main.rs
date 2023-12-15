

mod player;
mod main_state;

use ggez::{conf, event, ContextBuilder};
use main_state::MainState;

fn main() -> ggez::GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("top_down_shooter", "author")
        .window_setup(conf::WindowSetup::default().title("Top Down Shooter"))
        .window_mode(conf::WindowMode::default().dimensions(1200.0, 1200.0))
        .build()?;

    let state = MainState::new(&mut ctx)?;

    event::run(ctx, event_loop, state)
}

//This creates the context(window and eventloop) and then starts the loop running. 


//Why does main return GameResult? This is for error handling and common in Rust

//Breakdown the ContextBuilder.
//.window_setup: Configures window-related settings.
//.window_mode: Sets the mode/dimensions of the window.
//.build(): Finalizes the creation of the Context and EventLoop, returning them for use in your game.


//What the "?" do? Also related to error handling, the ? operator 
//is particularly useful for simplifying code that involves multiple operations 
//that can fail. Instead of using nested match statements to handle errors, 
//you can use ? for more concise and readable code.

