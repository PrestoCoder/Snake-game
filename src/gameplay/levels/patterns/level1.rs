use super::ObstaclePattern;

pub fn get_pattern(width: u16, height: u16) -> ObstaclePattern {
    let center_x = width / 2;
    let center_y = height / 2;
    let offset = 6;

    let positions = vec![
        // Center obstacle
        (center_x - 1, center_y - 1),
        // Top obstacle
        (center_x - 1, center_y - offset),
        // Bottom obstacle
        (center_x - 1, center_y + offset - 2),
        // Left obstacle
        (center_x - offset, center_y - 1),
        // Right obstacle
        (center_x + offset - 2, center_y - 1),
    ];

    // All obstacles are 2x2 in level 1
    let sizes = vec![(2, 2); 5];

    ObstaclePattern { positions, sizes }
}