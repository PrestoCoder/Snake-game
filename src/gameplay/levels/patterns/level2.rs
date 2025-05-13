use super::ObstaclePattern;

pub fn get_pattern(width: u16, height: u16) -> ObstaclePattern {
    let quarter_width = width / 4;
    let quarter_height = height / 4;
    let middle_width = width / 2 - 1;
    let middle_height = height / 2 - 1;

    let positions = vec![
        // Corner obstacles
        (quarter_width, quarter_height),
        (width - quarter_width - 2, quarter_height),
        (quarter_width, height - quarter_height - 2),
        (width - quarter_width - 2, height - quarter_height - 2),
        // Middle edge obstacles
        (middle_width, quarter_height),
        (middle_width, height - quarter_height - 2),
        (quarter_width, middle_height),
        (width - quarter_width - 2, middle_height),
    ];

    // All obstacles are 2x2 in level 2
    let sizes = vec![(2, 2); 8];

    ObstaclePattern { positions, sizes }
}