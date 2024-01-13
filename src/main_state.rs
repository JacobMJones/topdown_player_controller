use crate::collectible::Collectible;
use crate::collectible_placement;
use crate::collidable::Collidable;
use crate::event_handler::EventHandler;
use crate::player::Player;
use crate::proximity_and_collision_handler::handle_proximity_and_collisions;
use crate::smoke_effect::SmokeEffect;
use ggez::{event, graphics, Context, GameResult};
use crate::button::Button;
use gilrs::Gilrs;
use rand::Rng;
use mint::{Point2, Vector2};
const COLLECTIBLE_SIZE: f32 = 200.0;
const COLLECTIBLE_COUNT: i32 = 100;
const CLUSTER_SIZE: f32 = 200.0;
const PARTICLES_IN_SMOKE: i32 = 10;
const PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD: f32 = 800.0;
pub const PLAYER_POSITION_X: f32 = 100.0;
pub const PLAYER_POSITION_Y: f32 = 100.;
pub struct MainState {
    event_handler: EventHandler,
    player: Player,
    collectibles: Vec<Collectible>,
    smoke_effect_pool: Vec<SmokeEffect>,
    restart_button: Button,
    default_player_position: Point2<f32>
}

impl MainState {
    fn reset_collectibles(&mut self, ctx: &mut Context, screen_width: f32, screen_height: f32) {
        self.collectibles.clear(); // Clear existing collectibles
        self.collectibles = collectible_placement::generate_collectibles(
            ctx,
            screen_width,
            screen_height,
            COLLECTIBLE_COUNT,
            CLUSTER_SIZE,
            PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD,
            COLLECTIBLE_SIZE,
     
        ).expect("Failed to generate collectibles");
    }

    fn reset_player_position(&mut self) {
        self.player.position = self.default_player_position;
    }
    pub fn new(ctx: &mut Context, screen_width: f32, screen_height: f32) -> GameResult<MainState> {
        //gamepad
        let gilrs = Gilrs::new().unwrap();
        //gamepad events
        let event_handler = EventHandler::new(gilrs);
        let default_player_position = Point2 { x: PLAYER_POSITION_X, y: PLAYER_POSITION_Y };

        let button_width = 250.0;
        let button_height = 250.0;

        // Position the button at the bottom-center of the screen
        let button_x = 400.0; // center horizontally
        let button_y = 400.0; // 20 pixels from the bottom

        let restart_button = Button::new(
            ctx,
            Point2 { x: button_x, y: button_y },
            Vector2 { x: button_width, y: button_height },
            "Restart",
        )?;
        let collectibles = collectible_placement::generate_collectibles(
            ctx,
            screen_width,
            screen_height,
            COLLECTIBLE_COUNT,
            CLUSTER_SIZE,
            PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD,
            COLLECTIBLE_SIZE,
        )?;

        //Initialize multiple smoke effects and put them into a pool
        let mut smoke_effect_pool = Vec::new();
        for _ in 0..PARTICLES_IN_SMOKE {
            smoke_effect_pool.push(SmokeEffect::new_inactive());
        }

        let player = Player::new(Point2 { x: PLAYER_POSITION_X, y: PLAYER_POSITION_Y });

        Ok(MainState {
            event_handler,
            player,
            collectibles,
            smoke_effect_pool,
            restart_button,
            default_player_position
        })
    }

    fn handle_collectible_proximity(&mut self, collectible_index: usize, distance: f32) {
        if let Some(collectible) = self.collectibles.get_mut(collectible_index) {
            let in_proximity = distance < PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD / 2.0;
            collectible.set_in_proximity(
                in_proximity,
                distance,
                PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD,
            );
        }
    }

    fn handle_collectible_collision(
        &mut self,
        collectible_index: usize,
        to_remove: &mut Vec<usize>,
    ) {
        to_remove.push(collectible_index);
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: ggez::input::mouse::MouseButton, x: f32, y: f32) {
        // Check if the left mouse button was clicked
        
        if button == ggez::input::mouse::MouseButton::Left {

            if self.restart_button.is_clicked(mint::Point2 { x, y }) {
                self.reset_collectibles(ctx, 2000.0, 2000.0);
                self.reset_player_position();
                println!("button clicked");
          
            }
        }
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // Handle gamepad input
        self.event_handler.process_events(&mut self.player);

        // Get a trait object for player as collidable
        let player_collidable: &dyn Collidable = &self.player;

        // Check for proximity and collisions
        let proximity_and_collisions = handle_proximity_and_collisions(
            &[player_collidable],
            &self.collectibles.iter().collect::<Vec<&Collectible>>(),
            PLAYER_TO_COLLECTIBLE_PROXIMITY_THRESHOLD,
        );

        //set which collectibles should be removed due to collision
        let mut to_remove = Vec::new();

        for (_player_index, collectible_index, distance, is_collided) in proximity_and_collisions {
            self.handle_collectible_proximity(collectible_index, distance);
            if is_collided {
                self.handle_collectible_collision(collectible_index, &mut to_remove);
            }
        }
        // Remove the collectibles that collided with the player
        for index in to_remove.iter().rev() {
            if let Some(collectible) = self.collectibles.get_mut(*index) {
                collectible.activate_smoke_effect(&mut self.smoke_effect_pool);
                self.collectibles.remove(*index);
            }
        }
        // Update each collectible
        for collectible in &mut self.collectibles {
            collectible.player_direction = self.player.direction;
            collectible.update(ctx, dt, self.player.position);
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
            collectible.draw(ctx, self.player.position)?;
        }
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
