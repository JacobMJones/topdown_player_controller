use ggez::{graphics, Context, GameResult};
use ggez::graphics::Rect; 
use mint::Point2;
use ggez::graphics::Color;
use crate::flash_effect::FlashEffect;
use crate::collidable::Collidable;

pub struct Collectible {
    pub position: Point2<f32>,
    pub size: f32,
    pub active: bool,
    pub radius: f32,

}

impl Collectible {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Collectible {
            position: Point2 { x, y },
            size,
            active: true,
            radius: size/2.0,

        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.active {
            let square = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    self.position.x,
                    self.position.y,
                    self.size,
                    self.size,
                ),
                graphics::Color::from_rgb(55, 215, 0), // Gold color, for example
            )?;
            graphics::draw(ctx, &square, graphics::DrawParam::default())?;
        }
        Ok(())
    }
    pub fn bounding_box(&self) -> Rect {
        Rect::new(
            self.position.x - self.size / 2.0,
            self.position.y - self.size / 2.0,
            self.size,
            self.size,
        )
    }
    pub fn activate_flash_effect(&self, flash_effect_pool: &mut Vec<FlashEffect>) {
        if let Some(inactive_effect) = flash_effect_pool.iter_mut().find(|e| !e.is_active()) {
            let adjusted_position = Point2 {
                x: self.position.x + self.size / 2.0,
                y: self.position.y + self.size / 2.0,
            };

            inactive_effect.activate(
                adjusted_position,
                Color::new(1.0, 0.0, 0.0, 1.0), // Red color
                0.5, // Duration
            );
        }
    }
}

impl Collidable for Collectible {
    fn bounding_box(&self) -> Rect {
        self.bounding_box()
    }
}