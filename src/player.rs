use ggez::{graphics, Context, GameResult};
use mint;
use crate::collidable::Collidable;
use ggez::graphics::Rect; 
pub const MOVEMENT_SPEED: f32 = 1000.0;
//pub const ROTATION_SPEED: f32 = 3.0;

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
            position: mint::Point2 { x: 400.0, y: 300.0 },
            rotation: 0.0,
            axis_left: (0.0, 0.0),
            axis_right: (0.0, 0.0),
            speed: 0.0,
            acceleration: 2000.0, // Adjust this value as needed
            max_speed: MOVEMENT_SPEED,
            radius: 15.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Check if there is input on the left axis to start accelerating
        if self.axis_left.0 != 0.0 || self.axis_left.1 != 0.0 {
            self.speed += self.acceleration * dt;
            if self.speed > self.max_speed {
                self.speed = self.max_speed;
            }
        } else {
            // Decelerate or reset speed when there's no input
            self.speed = 0.0;
        }
    
        let movement = mint::Vector2 { 
            x: self.axis_left.0 * self.speed * dt, 
            y: self.axis_left.1 * self.speed * dt 
        };
    
        self.position.x += movement.x;
        self.position.y += movement.y;
    
        // Rotation logic (unchanged)
        if self.axis_right.0 != 0.0 || self.axis_right.1 != 0.0 {
            self.rotation = self.axis_right.1.atan2(self.axis_right.0);
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {

        // Define maximum and minimum radii
        let max_radius = 30.0;  // Larger radius when stationary
        let min_radius = 25.0;  // Smaller radius at maximum speed
        let radius_factor = self.speed / self.max_speed;
        let radius = max_radius - (max_radius - min_radius) * radius_factor;
        // Define white and yellow colors
        let white = graphics::Color::from_rgb(115, 215, 255);
        let yellow = graphics::Color::from_rgb(215, 255, 0);
    
        // Calculate the interpolation factor (0.0 when speed is 0, 1.0 when speed is max_speed)
        let factor = self.speed / self.max_speed;
    
        // Interpolate between white and yellow
        let color = graphics::Color::new(
            white.r + (yellow.r - white.r) * factor,
            white.g + (yellow.g - white.g) * factor,
            white.b + (yellow.b - white.b) * factor,
            1.0, // Alpha value
        );
    
        // Rest of your drawing code...
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            mint::Point2 { x: 0.0, y: 0.0 },
            radius,  // Use the interpolated radius
            2.0,
            color,   // Use the interpolated color
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