use ggez::{
    graphics::{self, Color, DrawMode, MeshBuilder},
    Context, GameResult,
};
use mint::Point2;
use noise::{NoiseFn, Perlin};

pub struct Tentacle {
    pub base_position: Point2<f32>,
    pub thickness: f32,
    pub color: Color,
    pub noise: Perlin,
    pub time: f64,
    pub points: Vec<Point2<f32>>,
    // Additional properties like noise scale, dynamic behavior, etc., can be added here.
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
        }
    }

    pub fn update(
        &mut self,
        ctx: &mut Context,
        target_position: Point2<f32>,
        max_length: f32,
        normalized_distance: f32,
        t: f32,
        color: Color,
    ) -> GameResult<()> {
        self.time = t as f64; // Increment time for noise evolution
                              // let modulated_time = self.time.cos() as f64;
        self.points.clear();
        // Calculate direction to the target
        let to_target = Point2 {
            x: target_position.x - self.base_position.x,
            y: target_position.y - self.base_position.y,
        };
        self.color = color;
        let distance_to_target = (to_target.x.powi(2) + to_target.y.powi(2)).sqrt();

        let direction = if distance_to_target != 0.0 {
            Point2 {
                x: to_target.x / distance_to_target,
                y: to_target.y / distance_to_target,
            }
        } else {
            to_target // If the target is exactly at the base position
        };

        // Calculate perpendicular direction for noise offset
        let perp_direction = Point2 {
            x: -direction.y,
            y: direction.x,
        };

        // Calculate the tentacle's length with a minimum length for visibility
        let min_tentacle_length = 2.0; // Minimum length can be the minimum for a visible line or more
        let max_tentacle_length = distance_to_target;
        
        let tentacle_length = (normalized_distance * max_tentacle_length).max(min_tentacle_length);

       // println!("ten len: {}, normal: {}, dist: {}, min_l: {}", tentacle_length, normalized_distance, distance_to_target, min_length);
       
        self.thickness = 20.0 * normalized_distance;
        // Generate points for the tentacle with noise
        //  let mut points = Vec::new();
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

        // // Build the tentacle mesh from the points
        // let mesh = MeshBuilder::new()
        //     .polyline(DrawMode::stroke(10.0), &points, self.color)?
        //     .build(ctx)?;

        // // Draw the tentacle mesh
        // graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;

        Ok(())
    }
    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // Build the tentacle mesh from the points
        let tentacle_mesh = MeshBuilder::new()
            .polyline(DrawMode::stroke(5.0), &self.points, self.color)?
            .build(ctx)?;

        // Draw the tentacle mesh
        graphics::draw(
            ctx,
            &tentacle_mesh,
            graphics::DrawParam::default(),
        )?;
        Ok(())
    }
    // Methods to allow changing the tentacle's properties, such as color and thickness
    pub fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }

    pub fn set_thickness(&mut self, new_thickness: f32) {
        self.thickness = new_thickness;
    }
}
