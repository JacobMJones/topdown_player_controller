use ggez::{graphics, Context, GameResult};
use mint;
use crate::collidable::Collidable;
use ggez::graphics::Rect; 
pub const MOVEMENT_SPEED: f32 = 1000.0;
pub const CIRCLE_SMOOTHNESS: f32 = 0.1;
pub const MAX_CIRCLE_RADIUS: f32 = 50.0;
pub const MIN_CIRCLE_RADIUS: f32 = 40.0;
pub const CIRCLE_RADIUS: f32 = 45.0;
pub const PLAYER_START_X_POS: f32 = 500.0;
pub const PLAYER_START_Y_POS: f32 = 500.0;
pub const PLAYER_ACCELERATION: f32 = 2000.0;

pub struct Player {
    pub position: mint::Point2<f32>,
    pub rotation: f32,
    pub axis_left: (f32, f32),
    pub axis_right: (f32, f32),
    pub speed: f32,
    pub acceleration: f32,
    pub max_speed: f32,
    pub radius: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: mint::Point2 { x: PLAYER_START_X_POS, y: PLAYER_START_Y_POS },
            rotation: 0.0,
            axis_left: (0.0, 0.0),
            axis_right: (0.0, 0.0),
            speed: 0.0,
            acceleration: PLAYER_ACCELERATION, // Adjust this value as needed
            max_speed: MOVEMENT_SPEED,
            radius: CIRCLE_RADIUS,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_movement(dt);
        self.update_rotation(dt);
    }

    fn update_movement(&mut self, dt: f32) {
        // Movement logic
        if self.axis_left.0 != 0.0 || self.axis_left.1 != 0.0 {
            self.speed += self.acceleration * dt;
            if self.speed > self.max_speed {
                self.speed = self.max_speed;
            }
        } else {
            self.speed = 0.0;
        }

        let movement = mint::Vector2 { 
            x: self.axis_left.0 * self.speed * dt, 
            y: self.axis_left.1 * self.speed * dt 
        };

        self.position.x += movement.x;
        self.position.y += movement.y;
    }

    fn update_rotation(&mut self, dt: f32) {
        if self.axis_right.0 != 0.0 || self.axis_right.1 != 0.0 {
            self.rotation = self.axis_right.1.atan2(self.axis_right.0);
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let radius = self.radius_based_on_speed();
        let color = self.color_based_on_speed();

        // Drawing the player
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 { x: 0.0, y: 0.0 },
            radius,  
            CIRCLE_SMOOTHNESS,
            color,   
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

    fn radius_based_on_speed(&self) -> f32 {
        let max_radius = MAX_CIRCLE_RADIUS;
        let min_radius = MIN_CIRCLE_RADIUS;
        let radius_factor = self.speed / self.max_speed;
        max_radius - (max_radius - min_radius) * radius_factor
    }

    fn color_based_on_speed(&self) -> graphics::Color {
        let white = graphics::Color::from_rgb(115, 215, 255);
        let yellow = graphics::Color::from_rgb(215, 255, 0);
        let factor = self.speed / self.max_speed;
        graphics::Color::new(
            white.r + (yellow.r - white.r) * factor,
            white.g + (yellow.g - white.g) * factor,
            white.b + (yellow.b - white.b) * factor,
            1.0, // Alpha value
        )
    }

}

impl Collidable for Player {
    fn bounding_box(&self) -> Rect {
        graphics::Rect::new(
            self.position.x - self.radius,
            self.position.y - self.radius,
            self.radius * 2.0,
            self.radius * 2.0,
        )
    }
}
