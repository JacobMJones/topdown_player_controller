use ggez::{graphics, Context, GameResult};
use mint;

pub const MOVEMENT_SPEED: f32 = 300.0;
pub const ROTATION_SPEED: f32 = 3.0;

pub struct Player {
    pub position: mint::Point2<f32>,
    pub rotation: f32,
    pub axis_left: (f32, f32),
    pub axis_right: (f32, f32),
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: mint::Point2 { x: 400.0, y: 300.0 },
            rotation: 0.0,
            axis_left: (0.0, 0.0),
            axis_right: (0.0, 0.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let movement = mint::Vector2 { 
            x: self.axis_left.0 * MOVEMENT_SPEED * dt, 
            y: self.axis_left.1 * MOVEMENT_SPEED * dt 
        };
        self.position.x += movement.x;
        self.position.y += movement.y;
        self.rotation += self.axis_right.0 * ROTATION_SPEED * dt;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 { x: 0.0, y: 0.0 },
            30.0,
            2.0,
            graphics::Color::WHITE,
        )?;

        graphics::draw(
            ctx,
            &circle,
            graphics::DrawParam::new()
                .dest(self.position)
                .rotation(self.rotation)
                .offset(mint::Point2 { x: 0.5, y: 0.5 }),
        )
    }

}