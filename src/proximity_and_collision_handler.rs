use crate::collidable::Collidable;
use ggez::graphics;

pub fn handle_proximity_and_collisions<T: Collidable + ?Sized, U: Collidable + ?Sized>(
    collidables1: &[&T], 
    collidables2: &[&U],
    proximity_threshold: f32
) -> Vec<(usize, usize, f32, bool)> {
    let mut results = Vec::new();

    for (i, collidable1) in collidables1.iter().enumerate() {
        let bbox1 = collidable1.bounding_box();

        for (j, collidable2) in collidables2.iter().enumerate() {
            let bbox2 = collidable2.bounding_box();

            let distance = distance_between(&bbox1, &bbox2);
            let is_collided = check_collision(&bbox1, &bbox2);

            if distance < proximity_threshold || is_collided {
                results.push((i, j, distance, is_collided));
            }
        }
    }

    results
}

pub fn check_collision(rect1: &graphics::Rect, rect2: &graphics::Rect) -> bool {
    rect1.x < rect2.x + rect2.w &&
    rect1.x + rect1.w > rect2.x &&
    rect1.y < rect2.y + rect2.h &&
    rect1.y + rect1.h > rect2.y
}

pub fn distance_between(rect1: &graphics::Rect, rect2: &graphics::Rect) -> f32 {
    let center1 = (rect1.x + rect1.w / 2.0, rect1.y + rect1.h / 2.0);
    let center2 = (rect2.x + rect2.w / 2.0, rect2.y + rect2.h / 2.0);
    let dx = center1.0 - center2.0;
    let dy = center1.1 - center2.1;
    (dx * dx + dy * dy).sqrt()
}
