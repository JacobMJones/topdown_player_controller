use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Color, DrawParam, Mesh};
use mint::Point2;

pub struct FlashEffect {
    position: Point2<f32>,
    color: Color,
    duration: f32, // Duration in seconds
    timer: f32,    // Timer to track the effect's lifetime
}

impl FlashEffect {
    pub fn new_inactive() -> Self {
        FlashEffect {
            position: Point2 { x: 0.0, y: 0.0 },
            color: Color::new(0.0, 0.0, 0.0, 0.0), // Transparent
            duration: 0.0,
            timer: 0.0,
        }
    }

    pub fn activate(&mut self, position: Point2<f32>, color: Color, duration: f32) {
        self.position = position;
        self.color = color;
        self.duration = duration;
        self.timer = 0.0; // Reset timer
    }

    pub fn update(&mut self, dt: f32) {
        if self.is_active() {
            self.timer += dt;

            // Calculate the remaining lifespan as a percentage
            let remaining_life = (self.duration - self.timer) / self.duration;
            // Update the alpha value to fade out
            self.color.a = remaining_life.clamp(0.0, 1.0);
        }
    }

    pub fn is_active(&self) -> bool {
        self.timer < self.duration && self.color.a > 0.0
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.is_active() {
            let mesh = Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                self.position,
                10.0, // radius of the flash effect
                0.01,  // tolerance, a lower value makes the circle smoother
                self.color,
            )?;

            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }
        Ok(())
    }
}
