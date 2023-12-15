use ggez::{event, Context, GameResult,graphics,};
use gilrs::{Event, Gilrs, EventType, Axis};

use crate::player::Player;

//Main state is passed context(ctx) from main.rs
pub struct MainState {
    gilrs: Gilrs,
    player: Player,
}
impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let gilrs = Gilrs::new().unwrap();
        let player = Player::new();

        Ok(MainState { gilrs, player })
    }
}
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

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

        self.player.update(dt);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(0, 0, 0));
        self.player.draw(ctx)?;
        graphics::present(ctx)
    }
}