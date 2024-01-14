use crate::button::Button;
use crate::collectibles::Collectibles;
use crate::event_handler::EventHandler;
use crate::player::Player;
use crate::smoke_effect::SmokeEffect;
use ggez::{event, graphics, Context, GameResult};
use gilrs::Gilrs;
use mint::{Point2, Vector2};
use rand::Rng;
const COLLECTIBLE_SIZE: f32 = 100.0;
const COLLECTIBLE_COUNT: i32 = 100;
const CLUSTER_SIZE: f32 = 200.0;
const PARTICLES_IN_SMOKE: i32 = 10;
const PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD: f32 = 800.0;
pub const PLAYER_POSITION_X: f32 = 100.0;
pub const PLAYER_POSITION_Y: f32 = 100.0;
pub struct MainState {
    event_handler: EventHandler,
    player: Player,
    // collectibles: Vec<Collectible>,
    smoke_effect_pool: Vec<SmokeEffect>,
    restart_button: Button,
    default_player_position: Point2<f32>,
    collectibles: Collectibles,
}

impl MainState {
    fn reset_player_position(&mut self) {
        self.player.position = self.default_player_position;
    }
    pub fn new(ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<MainState> {
        //gamepad
        let collectibles = Collectibles::new(
            ctx,
            screen_width,
            screen_height,
            COLLECTIBLE_COUNT,
            PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD,
            CLUSTER_SIZE,
            COLLECTIBLE_SIZE,
        )?; // Adjust parameters as needed

        let gilrs = Gilrs::new().unwrap();
        //gamepad events
        let event_handler = EventHandler::new(gilrs);
        let default_player_position = Point2 {
            x: PLAYER_POSITION_X,
            y: PLAYER_POSITION_Y,
        };

        let button_width = 150.0;
        let button_height = 50.0;

        // Position the button at the bottom-center of the screen
        let button_x = 1000.0; // center horizontally
        let button_y = 200.0; // 20 pixels from the bottom

        let restart_button = Button::new(
            ctx,
            Point2 {
                x: button_x,
                y: button_y,
            },
            Vector2 {
                x: button_width,
                y: button_height,
            },
            "Restart",
        )?;

        //Initialize multiple smoke effects and put them into a pool
        let mut smoke_effect_pool = Vec::new();
        for _ in 0..PARTICLES_IN_SMOKE {
            smoke_effect_pool.push(SmokeEffect::new_inactive());
        }

        let player = Player::new(Point2 {
            x: PLAYER_POSITION_X,
            y: PLAYER_POSITION_Y,
        });

        Ok(MainState {
            event_handler,
            player,
            collectibles,
            smoke_effect_pool,
            restart_button,
            default_player_position,
        })
    }


}

impl event::EventHandler<ggez::GameError> for MainState {
    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) {
        // Check if the left mouse button was clicked
        if button == ggez::input::mouse::MouseButton::Left {
            if self.restart_button.is_clicked(mint::Point2 { x, y }) {
                self.collectibles.reset_collectibles(ctx);
                self.reset_player_position();
                println!("button clicked");
            }
        }
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Handle gamepad input
        self.event_handler.process_events(&mut self.player);

        // Update Collectibles
        self.collectibles.update(ctx, dt, self.player.position, &self.player);

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

   
        self.collectibles.draw(ctx, self.player.position);

        // Draw each active smoke effect
        for effect in &self.smoke_effect_pool {
            if effect.is_active() {
                effect.draw(ctx)?;
            }
        }
        // Draw the player
        self.player.draw(ctx)?;

        //Draw UI
        self.restart_button.draw(ctx)?;
        graphics::present(ctx)
    }
}
