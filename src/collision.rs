use ggez::graphics;
use crate::player::Player;
use crate::collectible::Collectible;

pub fn check_collision(rect1: &graphics::Rect, rect2: &graphics::Rect) -> bool {
    rect1.x < rect2.x + rect2.w &&
    rect1.x + rect1.w > rect2.x &&
    rect1.y < rect2.y + rect2.h &&
    rect1.y + rect1.h > rect2.y
}

pub fn handle_collisions(player: &mut Player, collectibles: &mut Vec<Collectible>) -> Vec<usize> {
    let player_bbox = player.bounding_box();
    let mut to_remove = Vec::new();

    for (index, collectible) in collectibles.iter_mut().enumerate() {
        let collectible_bbox = collectible.bounding_box();
        if check_collision(&player_bbox, &collectible_bbox) {
            to_remove.push(index);
        }
    }

    to_remove
}