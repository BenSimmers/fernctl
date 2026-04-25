pub struct Furniture {
    pub x: u16,
    pub y: u16,
    pub art: &'static [&'static str],
    pub label: &'static str,
}

pub struct PlantPos {
    pub screen_x: u16,
    pub screen_y: u16,
    pub game_x: f32,
    pub game_y: f32,
}

pub const PLANTS: &[PlantPos] = &[
    PlantPos { screen_x: 4,  screen_y: 16, game_x: 7.0,  game_y: 18.0 },
    PlantPos { screen_x: 22, screen_y: 16, game_x: 25.0, game_y: 18.0 },
    PlantPos { screen_x: 40, screen_y: 16, game_x: 43.0, game_y: 18.0 },
    PlantPos { screen_x: 58, screen_y: 16, game_x: 61.0, game_y: 18.0 },
];

pub fn plant_art(thirst_ratio: f32) -> &'static [&'static str] {
    if thirst_ratio < 0.5 {
        &["  🌿    ", " \\│/   ", "  │    ", " ═══   "]
    } else if thirst_ratio < 0.8 {
        &["  🥀    ", "  \\│   ", "  │    ", " ═══   "]
    } else {
        &["  💀    ", "  ~~~  ", "  │    ", " ═══   "]
    }
}

pub fn office_furniture() -> Vec<Furniture> {
    vec![
        Furniture { x: 4,  y: 4, art: &["┌──────┐", "│ ⌨  ▪ │", "└──┬┬──┘", "   ││   "], label: "Desk 1" },
        Furniture { x: 22, y: 4, art: &["┌──────┐", "│ ⌨  ▪ │", "└──┬┬──┘", "   ││   "], label: "Desk 2" },
        Furniture { x: 40, y: 4, art: &["┌──────┐", "│ ⌨  ▪ │", "└──┬┬──┘", "   ││   "], label: "Desk 3" },
        Furniture { x: 58, y: 4, art: &["┌──────┐", "│ ⌨  ▪ │", "└──┬┬──┘", "   ││   "], label: "Desk 4" },
    ]
}

pub fn waypoints() -> Vec<(f32, f32)> {
    vec![
        // Near desks
        (7.0,  9.0),
        (25.0, 9.0),
        (43.0, 9.0),
        (61.0, 9.0),
        // Near plants
        (7.0,  21.0),
        (25.0, 21.0),
        (43.0, 21.0),
        (61.0, 21.0),
        // Hallway / wandering
        (15.0, 13.0),
        (35.0, 13.0),
        (50.0, 13.0),
        (30.0, 9.0),
    ]
}
