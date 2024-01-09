pub enum ClusterPointCollection {
    Collection1,
    Collection2,
    // ... other collections ...
}

pub fn get_cluster_points(
    collection: ClusterPointCollection,
    screen_width: f32,
    screen_height: f32,
) -> Vec<(f32, f32)> {
    match collection {
        ClusterPointCollection::Collection1 => vec![
            (screen_width * 0.07, screen_height * 0.5),
            (screen_width * 0.15, screen_height * 0.53),
            (screen_width * 0.2, screen_height * 0.58),
            (screen_width * 0.3, screen_height * 0.6),
            (screen_width * 0.4, screen_height * 0.65),
            (screen_width * 0.44, screen_height * 0.7),
            (screen_width * 0.5, screen_height * 0.74),
            (screen_width * 0.6, screen_height * 0.75),
            (screen_width * 0.65, screen_height * 0.8),
            (screen_width * 0.7, screen_height * 0.84),
            (screen_width * 0.75, screen_height * 0.88),
            (screen_width * 0.8, screen_height * 0.9),
            (screen_width * 0.85, screen_height * 0.92),
            (screen_width * 0.9, screen_height * 0.98),
        ],
        ClusterPointCollection::Collection2 => vec![
            (screen_width * 0.5, screen_height * 0.5),
            (screen_width * 0.5, screen_height * 0.53),
            (screen_width * 0.5, screen_height * 0.58),
            (screen_width * 0.5, screen_height * 0.6),
            (screen_width * 0.5, screen_height * 0.65),
            (screen_width * 0.5, screen_height * 0.7),
            (screen_width * 0.5, screen_height * 0.74),
            (screen_width * 0.5, screen_height * 0.75),
            (screen_width * 0.5, screen_height * 0.8),
            (screen_width * 0.5, screen_height * 0.84),
            (screen_width * 0.5, screen_height * 0.88),
            (screen_width * 0.5, screen_height * 0.9),
            (screen_width * 0.5, screen_height * 0.92),
            (screen_width * 0.5, screen_height * 0.98),
        ],
        // ... other collections ...
    }
}
