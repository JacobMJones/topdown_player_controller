use ggez::graphics;
use crate::player::Player;
//use crate::collectible::Collectible;
use crate::collidable::Collidable;


pub fn check_collision(rect1: &graphics::Rect, rect2: &graphics::Rect) -> bool {
    rect1.x < rect2.x + rect2.w &&
    rect1.x + rect1.w > rect2.x &&
    rect1.y < rect2.y + rect2.h &&
    rect1.y + rect1.h > rect2.y
}

pub fn handle_collisions<T: Collidable>(player: &Player, collidables: &[T]) -> Vec<usize> {
    let player_bbox = player.bounding_box();
    let mut to_remove = Vec::new();

    for (index, collidable) in collidables.iter().enumerate() {
        let collidable_bbox = collidable.bounding_box();
        if check_collision(&player_bbox, &collidable_bbox) {
            to_remove.push(index);
        }
    }

    to_remove
}
