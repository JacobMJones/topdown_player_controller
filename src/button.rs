use ggez::{graphics, Context, GameResult};
use mint::{Point2, Vector2};

pub struct Button {
    rect: graphics::Rect,
    text: graphics::Text,
    pub is_hovered: bool,
}

impl Button {
    pub fn new(ctx: &mut Context, position: Point2<f32>, size: Vector2<f32>, label: &str) -> GameResult<Button> {
        let rect = graphics::Rect::new(position.x, position.y, size.x, size.y);
        let text = graphics::Text::new(label);
        Ok(Button {
            rect,
            text,
            is_hovered: false,
        })
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let color = if self.is_hovered {
            graphics::Color::from_rgb(200, 200, 200)
        } else {
            graphics::Color::from_rgb(255, 255, 255)
        };
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.rect,
            color,
        )?;
        graphics::draw(ctx, &rectangle, graphics::DrawParam::default())?;
        graphics::draw(ctx, &self.text, (Point2 { x: self.rect.x, y: self.rect.y },))
    }

    pub fn check_mouse_hover(&mut self, mouse_position: Point2<f32>) {
        self.is_hovered = self.rect.contains(Point2 { x: mouse_position.x, y: mouse_position.y });
    }

    pub fn is_clicked(&self, mouse_position: Point2<f32>) -> bool {
        let within_x_bounds = mouse_position.x >= self.rect.x && mouse_position.x <= (self.rect.x + self.rect.w);
        let within_y_bounds = mouse_position.y >= self.rect.y && mouse_position.y <= (self.rect.y + self.rect.h);
    
        println!("Within X bounds: {}", within_x_bounds);
        println!("Within Y bounds: {}", within_y_bounds);
    
        within_x_bounds && within_y_bounds
    }
}
