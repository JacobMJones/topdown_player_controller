use ggez::graphics::{Color, MeshBuilder};
use ggez::{graphics, Context, GameResult};
use mint::{Point2, Vector2};
pub struct Eye {
    position: Point2<f32>,
    scale: f32,
    color: Color,
    in_proximity: bool,
    distance_from_player: f32
}

impl Eye {
    pub fn new(x: f32, y: f32, scale: f32) -> Self {
        Eye {
            position: Point2 { x, y },
            scale,
            color: Color::WHITE,
            in_proximity: false,
            distance_from_player: 0.0,
        }
    }
    // Draw the eye on the screen
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if !self.in_proximity {
            let circle_mesh = MeshBuilder::new()
                .circle(
                    graphics::DrawMode::fill(),
                    [self.position.x, self.position.y],
                    self.scale,
                    0.2,
                    self.color,
                )?
                .build(ctx)?;
    
            graphics::draw(ctx, &circle_mesh, graphics::DrawParam::default())?;
    
            let pupil_scale = self.scale * 0.3;
            let pupil_color = Color::BLACK;
    
            let pupil_mesh = MeshBuilder::new()
                .circle(
                    graphics::DrawMode::fill(),
                    [self.position.x, self.position.y],
                    pupil_scale,
                    0.1,
                    pupil_color,
                )?
                .build(ctx)?;
    
            graphics::draw(ctx, &pupil_mesh, graphics::DrawParam::default())
        } else {
            Ok(()) // Explicitly return Ok(()) if nothing should be drawn
        }
    }
    pub fn update(
        &mut self,
        player_position: Point2<f32>,
        collectible_center: Point2<f32>,
        distance_from_player: f32,
        in_proximity: bool,
    ) {
        // Calculate the direction towards the player
       // println!("distance from {} {}", distance_from_player, in_proximity);
        self.in_proximity = in_proximity;
        self.distance_from_player = distance_from_player;
        if !in_proximity {
            let to_player = Vector2 {
                x: player_position.x - collectible_center.x,
                y: player_position.y - collectible_center.y,
            };
            let distance_to_player = (to_player.x.powi(2) + to_player.y.powi(2)).sqrt();
            let direction = if distance_to_player != 0.0 {
                Vector2 {
                    x: to_player.x / distance_to_player,
                    y: to_player.y / distance_to_player,
                }
            } else {
                Vector2 { x: 0.0, y: 0.0 } // No movement if player is exactly at the collectible center
            };

            // Define the maximum offset the eye can have from the center
            let max_eye_offset = self.scale * 4.0; // for instance, 30% of eye's scale, adjust as needed

            // Calculate the actual offset
            let eye_offset = Vector2 {
                x: direction.x * max_eye_offset,
                y: direction.y * max_eye_offset,
            };

            // Update eye's position to be a slight offset from the collectible's center
            self.position = Point2 {
                x: collectible_center.x + eye_offset.x,
                y: collectible_center.y + eye_offset.y,
            };
        }
    }

    // Updates the scale (size) of the eye
    pub fn set_scale(&mut self, new_scale: f32) {
        self.scale = new_scale;
    }

    // Updates the color of the eye
    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }
}
