use ggez::{graphics, Context, GameResult};
use mint;

pub const MOVEMENT_SPEED: f32 = 800.0;
// pub const ROTATION_SPEED: f32 = 3.0;

pub struct Player {
    pub position: mint::Point2<f32>,
    pub rotation: f32,
    pub axis_left: (f32, f32),
    pub axis_right: (f32, f32),
    pub speed: f32,
    pub acceleration: f32,
    pub max_speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: mint::Point2 { x: 400.0, y: 300.0 },
            rotation: 0.0,
            axis_left: (0.0, 0.0),
            axis_right: (0.0, 0.0),
            speed: 0.0,
            acceleration: 1000.0, // Adjust this value as needed
            max_speed: MOVEMENT_SPEED,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let axis_input = mint::Vector2 {
            x: self.axis_left.0,
            y: self.axis_left.1,
        };

        let input_magnitude = (axis_input.x.powi(2) + axis_input.y.powi(2)).sqrt();

        // Check if there is input to start accelerating
        if input_magnitude != 0.0 {
            self.speed += self.acceleration * dt;
            if self.speed > self.max_speed {
                self.speed = self.max_speed;
            }

            let normalized_input = mint::Vector2 {
                x: axis_input.x / input_magnitude,
                y: axis_input.y / input_magnitude,
            };

            let movement = mint::Vector2 {
                x: normalized_input.x * self.speed * dt,
                y: normalized_input.y * self.speed * dt,
            };

            self.position.x += movement.x;
            self.position.y += movement.y;
        } else {
            // Decelerate or reset speed when there's no input
            self.speed = 0.0;
        }
    

        if self.axis_right.0 != 0.0 || self.axis_right.1 != 0.0 {
            self.rotation = self.axis_right.1.atan2(self.axis_right.0);
        }
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