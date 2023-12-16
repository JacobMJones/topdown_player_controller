use ggez::{graphics, Context, GameResult};
use ggez::graphics::Rect; // Import Rect
use mint::Point2;
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
            self.position.x - self.radius,
            self.position.y - self.radius,
            self.size,
            self.size,
        )
    }

}
