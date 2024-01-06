use crate::collidable::Collidable;
use crate::smoke_effect::SmokeEffect;
use ggez::graphics::{self, Color, Mesh, DrawMode, MeshBuilder, Rect};
use ggez::{Context, GameResult};
use mint::Point2;
use noise::{NoiseFn, Perlin};

pub struct Collectible {
    pub position: Point2<f32>,
    pub size: f32,
    pub active: bool,
    pub radius: f32,
    pub time: f32,
    pub id: String,
    pub in_proximity: bool,
    pub distance_from_player: f32,
    mesh: Mesh,
    noise: Perlin,
}

impl Collectible {
    pub fn new(ctx: &mut Context, x: f32, y: f32, size: f32, initial_time: f32, id: String, in_proximity: bool, distance_from_player:f32) -> GameResult<Self> {
        let noise = Perlin::new();
        let mesh = create_amorphous_mesh(ctx, size, &noise, initial_time, in_proximity, distance_from_player)?;
        Ok(Collectible { 
            position: Point2 { x, y },
            size,
            active: true,
            radius: size / 2.0,
            time: initial_time,
            id,
            in_proximity: false,
            distance_from_player,
            mesh,
            noise
        })
    }

    pub fn update(&mut self, ctx: &mut Context, dt: f32) -> GameResult<()> {
        self.time += dt;
        self.mesh = create_amorphous_mesh(ctx, self.size, &self.noise, self.time, self.in_proximity, self.distance_from_player)?;
        Ok(())
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.active {
           // let size = self.get_pulsating_size();
            let size = self.size;
            let color = self.get_dynamic_color();

            let scale_x = size / self.size;
            let scale_y = size / self.size;
          
            graphics::draw(
                ctx,
                &self.mesh,
                graphics::DrawParam::default()
                    .dest([self.position.x, self.position.y])
                    .scale([scale_x, scale_y])
                    .color(color),
            )?;
        }
        Ok(())
    }

    pub fn bounding_box(&self) -> Rect {
        Rect::new(
            self.position.x - self.size / 2.0,
            self.position.y - self.size / 2.0,
            self.size,
            self.size,
        )
    }

    pub fn activate_smoke_effect(&self, smoke_effect_pool: &mut Vec<SmokeEffect>) {
        let base_position = Point2 {
            x: self.position.x + self.size / 2.0,
            y: self.position.y + self.size / 2.0,
        };
        for _ in 0..5 {
            if let Some(inactive_effect) = smoke_effect_pool.iter_mut().find(|e| !e.is_active()) {
                inactive_effect.activate(base_position);
            }
        }
    }

    
    fn get_dynamic_color(&self) -> Color {
        if !self.in_proximity {
            let g = ((self.time.sin() * 0.25 + 0.75) * 0.5 + 0.5) as f32;
            let b = 0.0 as f32;
            let r = 1.0 as f32;
           // Color::new(1.0, 0.4, 1.0, 1.0)

            

             Color::new(r, g, b, 1.0)
        } else {
            let g = ((self.time.sin() * 0.25 + 0.75) * 0.5 + 0.5) as f32;
            let b = 1.0 as f32;
            let r = 0.0 as f32;
            Color::new(r,g, 1.0, 1.0)
        }
    }

    pub fn set_in_proximity(&mut self, in_proximity: bool, distance: f32) {
        
        self.in_proximity = in_proximity;
        //self.distance_from_player = distance
        self.distance_from_player = 1.0 - (distance - 50.0) / (1000.0 - 50.0); //normalized between 1 and 0
    }
}

impl Collidable for Collectible {
    fn bounding_box(&self) -> Rect {
        self.bounding_box()
    }
}

fn create_amorphous_mesh(ctx: &mut Context, size: f32, noise: &Perlin, time: f32, in_proximity: bool, distance_from_player:f32) -> GameResult<Mesh> {
   
    println!("in prox {}, distance {}", in_proximity, distance_from_player);

    let mut builder = MeshBuilder::new();
     // Normalized between 0 and 1
    
    println!("distance from player {}", distance_from_player);
    let num_points = 20;
    let angle_step = (2.1) * std::f32::consts::PI / num_points as f32;

    let noise_scale = if in_proximity { 0.6 } else { 0.3 };
    let time_scale = if in_proximity { 0.6 + (distance_from_player/10.0)} else { 0.2 };
    
    let mut points = Vec::new();

    let base_radius = 50.0; // 
    let min_radius = base_radius * 0.4; 
    let noise_amplitude = base_radius * distance_from_player; 

    // First pass: calculate points for the blob
    for i in 0..num_points {
        let angle = i as f32 * angle_step;
        let noise_x = (angle.cos() * noise_scale + time * time_scale) as f64;
        let noise_y = (angle.sin() * noise_scale + time * time_scale) as f64;
        let noise_value = noise.get([noise_x, noise_y]) as f32;
        let noise_offset = noise_value * noise_amplitude;
        let radius = (base_radius + noise_offset).max(min_radius);
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        
        points.push(Point2 { x, y });
    }

    // Second pass: smooth the points
    let smoothed_points = smooth_points(&points);

    // Build the polygon with smoothed points
    builder.polygon(
        DrawMode::fill(),
        &smoothed_points,
        Color::from_rgb(255, 255, 255), 
    )?;

    builder.build(ctx)
}

fn smooth_points(points: &[Point2<f32>]) -> Vec<Point2<f32>> {
    let len = points.len();
    let mut smoothed_points = Vec::with_capacity(len);

    for i in 0..len {
        let prev = if i == 0 { points[len - 1] } else { points[i - 1] };
        let next = if i == len - 1 { points[0] } else { points[i + 1] };
        let current = points[i];

        let avg_x = (prev.x + current.x + next.x) / 3.0;
        let avg_y = (prev.y + current.y + next.y) / 3.0;

        smoothed_points.push(Point2 { x: avg_x, y: avg_y });
    }

    smoothed_points
}
