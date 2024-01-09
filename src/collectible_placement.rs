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
    let mut collectibles = Vec::new();
    let mut rng = rand::thread_rng();

    let cluster_points = collectible_cluster_points::get_cluster_points(
        ClusterPointCollection::Collection2,
        screen_width,
        screen_height,
    );

    for i in 0..collectible_count {
        // Choose a random cluster point
        let (center_x, center_y) = cluster_points[rng.gen_range(0..cluster_points.len())];

        // Generate positions near the cluster point
        let x = rng
            .gen_range(center_x - cluster_size..=center_x + cluster_size)
            .clamp(50.0, screen_width - 50.0);  // Assuming 50.0 is the margin
        let y = rng
            .gen_range(center_y - cluster_size..=center_y + cluster_size)
            .clamp(50.0, screen_height - 50.0); // Assuming 50.0 is the margin

        // Add randomness to the shapeshifting start point
        let initial_time = rng.gen_range(0.0..6.28);

        let id = format!("collect{}", i);

        // Create a new collectible
        let collectible = Collectible::new(
            ctx,
            x,
            y,
            collectible_size,
            proximity_threshold,
            initial_time,
            id,
            false, // Assuming some default values
            100.0, // Assuming some default value for score or similar
        )?;

        // Push the newly created collectible to the vector
        collectibles.push(collectible);
    }

    Ok(collectibles)
}