use ggez::graphics;

pub trait Collidable {
    fn bounding_box(&self) -> graphics::Rect;
}