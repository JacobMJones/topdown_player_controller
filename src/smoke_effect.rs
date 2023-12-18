use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Color, DrawParam, Mesh};
use mint::Point2;
use rand::Rng; 
pub const OFFSET_RANGE: f32 = 2.0;
pub const DURATION: f32 = 0.2;
pub const SMOKE_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const SMOKE_CIRCLE_RADIUS: f32 = 15.0;
pub const SMOKE_CIRCLE_SMOOTHNESS: f32 = 1.0;

pub struct SmokeEffect {
    position: Point2<f32>,
    color: Color,
    duration: f32, 
    timer: f32,    
}

impl SmokeEffect {
    pub fn new_inactive() -> Self {
        SmokeEffect {
            position: Point2 { x: 0.0, y: 0.0 },
            color: SMOKE_COLOR, 
            duration: DURATION,
            timer: 0.0,
        }
    }

    pub fn activate(&mut self, base_position: Point2<f32>) {
        let mut rng = rand::thread_rng();
        let offset_x: f32 = rng.gen_range(-OFFSET_RANGE..OFFSET_RANGE);
        let offset_y: f32 = rng.gen_range(-OFFSET_RANGE..OFFSET_RANGE);

        self.position = Point2 {
            x: base_position.x + offset_x,
            y: base_position.y + offset_y,
        };
        self.color = SMOKE_COLOR;
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
                SMOKE_CIRCLE_RADIUS, 
                SMOKE_CIRCLE_SMOOTHNESS,  
                self.color,
            )?;

            graphics::draw(ctx, &mesh, DrawParam::default())?;
        }
        Ok(())
    }
}
