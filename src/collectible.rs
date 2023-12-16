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
    pub time: f32,
}

impl Collectible {
    pub fn new(x: f32, y: f32, size: f32, initial_time:f32) -> Self {
        Collectible {
            position: Point2 { x, y },
            size,
            active: true,
            radius: size/2.0,
            time:initial_time
        }
    }
    fn get_pulsating_size(&self) -> f32 {
        let pulsation_factor = 0.5; // Adjust this value for more/less pulsation
        let min_size = 10.0; // Minimum size
        let max_size = self.size; // Maximum size, based on initial size
        let mut pulsating_size = self.size + pulsation_factor * self.time.sin();

        // Ensure the size is within the min and max bounds
        if pulsating_size < min_size {
            pulsating_size = min_size + (min_size - pulsating_size);
        } else if pulsating_size > max_size {
            pulsating_size = max_size - (pulsating_size - max_size);
        }

        pulsating_size
    }
    fn get_dynamic_color(&self) -> Color {
        let r = (self.time.sin() * 0.5 + 0.5) as f32;
        let g = ((self.time + 2.0).sin() * 0.5 + 0.5) as f32;
        let b = ((self.time + 4.0).sin() * 0.5 + 0.5) as f32;
        Color::new(r, g, b, 1.0)
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.active {
            let size = self.get_pulsating_size(); 
            let square = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    self.position.x,
                    self.position.y,
                    size,
                    size,
                ),
                self.get_dynamic_color(), 
                // graphics::Color::from_rgb(55, 215, 0), 

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