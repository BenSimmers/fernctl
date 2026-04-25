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

/// Generate `n` plants at random positions spread across the screen.
/// Divides the usable x-axis into `n` equal bands so plants are distributed.
pub fn generate_plants(n: usize, term_width: u16, term_height: u16) -> Vec<PlantPos> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    // Leave margins: 2 cols on each side, avoid title (y=0) and status bar (y=last)
    // Also avoid the desk row (y ~4..9) — place plants in the lower 60% of screen
    let usable_x = term_width.saturating_sub(10); // leave room for 8-char art + margins
    let y_min = (term_height as f32 * 0.45) as u16;
    let y_max = term_height.saturating_sub(5).max(y_min + 1);
    let band_width = usable_x / n as u16;
    (0..n)
        .map(|i| {
            let min_x = 2 + i as u16 * band_width;
            let max_x = (min_x + band_width).saturating_sub(9).max(min_x);
            let sx = rng.gen_range(min_x..=max_x);
            let sy = rng.gen_range(y_min..=y_max);
            PlantPos {
                screen_x: sx,
                screen_y: sy,
                game_x: sx as f32 + 3.0,
                game_y: sy as f32 + 2.0,
            }
        })
        .collect()
}

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

pub fn waypoints(plants: &[PlantPos]) -> Vec<(f32, f32)> {
    let mut wps = vec![
        // Near desks
        (7.0,  9.0),
        (25.0, 9.0),
        (43.0, 9.0),
        (61.0, 9.0),
        // Hallway / wandering
        (15.0, 13.0),
        (35.0, 13.0),
        (50.0, 13.0),
        (30.0, 9.0),
    ];
    // Near plants (dynamic positions)
    for plant in plants {
        wps.push((plant.game_x, plant.game_y + 3.0));
    }
    wps
}
