use ggez::{
    graphics::{self, Color, DrawMode, MeshBuilder},
    Context, GameResult,
};
use mint::Point2;
use noise::{NoiseFn, Perlin};
use nalgebra::{Vector2, Norm};
pub struct Tentacle {
    pub base_position: Point2<f32>,
    pub thickness: f32,
    pub color: Color,
    pub noise: Perlin,
    pub time: f64,
    pub points: Vec<Point2<f32>>,
    pub in_proximity: bool,

}

impl Tentacle {
    pub fn new(
        base_position: Point2<f32>,
        thickness: f32,
        color: Color,
        initial_time: f64,
    ) -> Self {
        Tentacle {
            base_position,
            thickness,
            color,
            time: initial_time,
            noise: Perlin::new(),
            points: Vec::new(),
            in_proximity: false,
        }
    }

    pub fn update(
        &mut self,
        _ctx: &mut Context,
        target_position: Point2<f32>,
        _max_length: f32,
        normalized_distance: f32,
        t: f32,
        color: Color,
        in_proximity: bool,
        max_distance_threshold: f32,
    ) -> GameResult<()> {
        self.in_proximity = in_proximity;
        self.color = color;
        self.time = t as f64; // time for noise evolution  // let modulated_time = self.time.cos() as f64;
        self.points.clear();

        let base_position = Vector2::new(self.base_position.x, self.base_position.y);
        let target_position = Vector2::new(target_position.x, target_position.y);
        let to_target = target_position - base_position;
        let distance_to_target = to_target.norm(); 
        //Distance and direction to target


        let direction = if distance_to_target != 0.0 {
            to_target.normalize()  // normalize the vector
        } else {
            Vector2::new(0.0, 0.0) // Or handle the zero distance case as you prefer
        };
        
        // Calculate perpendicular direction for noise offset
        let perp_direction = Vector2::new(-direction.y, direction.x);
        // Tentacle length, thickness and points --- ISSUE, sometimes if the player moves away too quickly the tentacle remains
        let min_tentacle_length = 2.0;
        let max_tentacle_length = 
            distance_to_target.clamp(min_tentacle_length, max_distance_threshold) * 1.2;
        let tentacle_length = (normalized_distance * max_tentacle_length).max(min_tentacle_length);

        self.thickness = 10.0;

        // Generate points for the tentacle with noise
        for i in 0..=tentacle_length as usize {
            let along = i as f32 / tentacle_length; // Normalized position along tentacle
            let noise_value = self.noise.get([self.time + along as f64 * 2.0, 0.0]) as f32;
            let noise_offset = noise_value * 23.0; // Scale the noise effect

            // Calculate the vertex position with noise
            let vertex = Point2 {
                x: self.base_position.x + direction.x * i as f32 + perp_direction.x * noise_offset,
                y: self.base_position.y + direction.y * i as f32 + perp_direction.y * noise_offset,
            };
            self.points.push(vertex);
        }

        Ok(())
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // Build the tentacle mesh from the points
        let tentacle_mesh = MeshBuilder::new()
            .polyline(DrawMode::stroke(self.thickness), &self.points, self.color)?
            .build(ctx)?;

        graphics::draw(ctx, &tentacle_mesh, graphics::DrawParam::default())?;
        Ok(())
    }

    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }

    pub fn set_thickness(&mut self, new_thickness: f32) {
        self.thickness = new_thickness;
    }
}