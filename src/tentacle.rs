use ggez::{
    graphics::{self, Color, DrawMode, MeshBuilder},
    Context, GameResult,
};
use mint::Point2;
use nalgebra::{Norm, Vector2};
use noise::{NoiseFn, Perlin};
#[derive(Debug)]
pub struct Tentacle {
    pub base_position: Point2<f32>,
    pub thickness: f32,
    pub color: Color,
    pub noise: Perlin,
    pub time: f64,
    pub points: Vec<Point2<f32>>,
    pub in_proximity: bool,
    current_tentacle_length: f32,
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
            current_tentacle_length: 1.0,
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
        self.time = t as f64; // time for noise evolution
        self.points.clear();
    
        let base_position = Vector2::new(self.base_position.x, self.base_position.y);
        let target_position = Vector2::new(target_position.x, target_position.y);
        let to_target = target_position - base_position;
        let distance_to_target = to_target.norm();
    
        let direction = if distance_to_target != 0.0 {
            to_target.normalize()
        } else {
            Vector2::new(0.0, 0.0)
        };
    
        let perp_direction = Vector2::new(-direction.y, direction.x);
    
        let min_tentacle_length = 1.0;
        let max_tentacle_length = distance_to_target.clamp(min_tentacle_length, max_distance_threshold);
    
        // Calculate the desired length of the tentacle based on proximity
        let desired_length = if in_proximity {
            (normalized_distance * max_tentacle_length).max(min_tentacle_length)
        } else {
            // When not in proximity, gradually retract to min length
            self.current_tentacle_length * 0.95 // Retract speed, adjust as needed
        };
    
        // Gradually adjust the current length of the tentacle towards the desired length
        self.current_tentacle_length = if self.current_tentacle_length < desired_length {
            self.current_tentacle_length + (desired_length - self.current_tentacle_length) * 0.05 // Grow speed, adjust as needed
        } else {
            self.current_tentacle_length - (self.current_tentacle_length - desired_length) * 0.05 // Retract speed, adjust as needed
        };
    
        // Generate points for the tentacle with noise
        for i in 0..=self.current_tentacle_length as usize {
            let along = i as f32 / self.current_tentacle_length; // Normalized position along tentacle
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
