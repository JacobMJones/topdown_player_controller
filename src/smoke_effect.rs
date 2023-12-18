use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Color, DrawParam, Mesh};
use mint::Point2;
use rand::Rng; 
pub struct SmokeEffect {
    position: Point2<f32>,
    color: Color,
    duration: f32, // Duration in seconds
    timer: f32,    // Timer to track the effect's lifetime
}

impl SmokeEffect {
    pub fn new_inactive() -> Self {
        SmokeEffect {
            position: Point2 { x: 0.0, y: 0.0 },
            color: Color::new(0.0, 0.0, 0.0, 0.0), // Transparent
            duration: 0.0,
            timer: 0.0,
        }
    }

    pub fn activate(&mut self, base_position: Point2<f32>, offset_range: f32, color: Color, duration: f32) {
        let mut rng = rand::thread_rng();
        let offset_x: f32 = rng.gen_range(-offset_range..offset_range);
        let offset_y: f32 = rng.gen_range(-offset_range..offset_range);

        self.position = Point2 {
            x: base_position.x + offset_x,
            y: base_position.y + offset_y,
        };
        self.color = color;
        self.duration = duration;
        self.timer = 0.0; 
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
                10.0, // radius of the smoke effect
                0.01,  // tolerance, a lower value makes the circle smoother
                self.color,
            )?;

            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }
        Ok(())
    }
}
