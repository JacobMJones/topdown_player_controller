use crate::collectible::Collectible;
use crate::collectible_cluster_points::{self, ClusterPointCollection};
use ggez::Context;
use rand::Rng;

pub fn generate_collectibles(
    ctx: &mut Context,
    screen_width: f32,
    screen_height: f32,
    collectible_count: i32,
    cluster_size: f32,
    proximity_threshold: f32,
    collectible_size: f32,
) -> ggez::GameResult<Vec<Collectible>> {
    let mut collectibles: Vec<Collectible> = Vec::new();
    let mut rng = rand::thread_rng();

    let cluster_points = collectible_cluster_points::get_cluster_points(
        ClusterPointCollection::Collection2,
        screen_width,
        screen_height,
    );

    for i in 0..collectible_count {

        let mut position_ok = false;
        let mut failure_count = 0; 
   
        // Add randomness to the shapeshifting start point
        let initial_time = rng.gen_range(0.0..6.28);
        let id = format!("collect{}", i);

        while !position_ok {
            position_ok = true;
            let (center_x, center_y) = cluster_points[rng.gen_range(0..cluster_points.len())];
            let x = rng
                .gen_range(center_x - cluster_size..=center_x + cluster_size)
                .clamp(20.0, screen_width - 20.0); // Assuming 50.0 is the margin
            let y = rng
                .gen_range(center_y - cluster_size..=center_y + cluster_size)
                .clamp(20.0, screen_height - 20.0); // Assuming 50.0 is the margin



            for existing_collectible in &collectibles {
                let distance = ((existing_collectible.position.x - x).powi(2)
                    + (existing_collectible.position.y - y).powi(2))
                .sqrt();
                if distance < 3.0 {
                   
                    position_ok = false;
                    failure_count += 1;
                    println!("too close {} {}", distance, position_ok);
                    break;
                }
            }
            if failure_count >= 20 {
                println!("Error: Failed to place collectible after 20 attempts");
                return Err(ggez::GameError::CustomError("Failed to place collectible".to_string()));
            }
            if position_ok {
                
                failure_count = 0;
                let collectible = Collectible::new(
                    ctx,
                    x,
                    y,
                    collectible_size,
                    proximity_threshold,
                    initial_time,
                    id.clone(),
                    false,   // Assuming some default values
                    10000.0, // Assuming some default value for score or similar
                )?;




                collectibles.push(collectible);
            }

        }

    }
    // for i in 0..collectibles.len() {
    //     println!("{:?}", collectibles[i]);
    // }
    Ok(collectibles)
}
