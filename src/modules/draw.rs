use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::modules::app::{App, PLANT_MAX_THIRST};
use crate::modules::character::{Character, ContainerState};
use crate::modules::office::{office_furniture, plant_art, Furniture, PlantPos};

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    // Background — dark office floor
    for y in 0..area.height {
        let floor_char = if (y / 2) % 2 == 0 { "·" } else { " " };
        let line_str: String = (0..area.width)
            .map(|x| if (x + y) % 4 == 0 { floor_char } else { " " })
            .collect();
        let p = Paragraph::new(Line::from(Span::styled(
            line_str,
            Style::default().fg(Color::Rgb(40, 40, 50)),
        )));
        f.render_widget(p, Rect { x: 0, y, width: area.width, height: 1 });
    }

    // Title bar
    let title = format!(
        " 🏢 Docker Office  —  {} containers  │  Tab cycle  ↑↓←→ move  Space water  r refresh  q quit ",
        app.containers.len()
    );
    let title_p = Paragraph::new(Line::from(Span::styled(
        title,
        Style::default()
            .fg(Color::White)
            .bg(Color::Rgb(30, 30, 60))
            .add_modifier(Modifier::BOLD),
    )));
    f.render_widget(title_p, Rect { x: 0, y: 0, width: area.width, height: 1 });

    // Draw walls
    draw_walls(f, area);

    // Draw furniture (desks only; plants are drawn separately)
    let furniture = office_furniture();
    for furn in &furniture {
        draw_furniture_piece(f, furn, area);
    }
    for (i, plant) in app.plants.iter().enumerate() {
        let thirst_ratio = app.plant_thirsts[i] as f32 / PLANT_MAX_THIRST as f32;
        draw_plant(f, plant, thirst_ratio, area);
    }

    // Draw characters
    for (i, ch) in app.characters.iter().enumerate() {
        draw_character(f, ch, area, i == app.selected, app.tick);
    }

    // Status bar at bottom
    draw_status_bar(f, app, area);

    // If no containers, show message
    if app.containers.is_empty() {
        let msg =
            "🏚  No containers found — the office is empty! Start some containers and press 'r'";
        let mx = area.width.saturating_sub(msg.len() as u16) / 2;
        let my = area.height / 2;
        let p = Paragraph::new(Line::from(Span::styled(
            msg,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        f.render_widget(
            p,
            Rect {
                x: mx,
                y: my,
                width: msg.len() as u16 + 1,
                height: 1,
            },
        );
    }
}

fn draw_walls(f: &mut Frame, area: Rect) {
    let wall_style = Style::default().fg(Color::Rgb(80, 80, 100));

    // Top wall (below title)
    let top_wall: String = "═".repeat(area.width as usize);
    let p = Paragraph::new(Line::from(Span::styled(top_wall, wall_style)));
    f.render_widget(p, Rect { x: 0, y: 1, width: area.width, height: 1 });

    // Bottom wall (above status)
    let bot_wall: String = "═".repeat(area.width as usize);
    let p = Paragraph::new(Line::from(Span::styled(bot_wall, wall_style)));
    f.render_widget(
        p,
        Rect {
            x: 0,
            y: area.height.saturating_sub(2),
            width: area.width,
            height: 1,
        },
    );

    // Side walls
    for y in 2..area.height.saturating_sub(2) {
        let p = Paragraph::new(Line::from(Span::styled("║", wall_style)));
        f.render_widget(p, Rect { x: 0, y, width: 1, height: 1 });
        let p = Paragraph::new(Line::from(Span::styled("║", wall_style)));
        f.render_widget(
            p,
            Rect {
                x: area.width.saturating_sub(1),
                y,
                width: 1,
                height: 1,
            },
        );
    }

    // Corners
    let p = Paragraph::new(Line::from(Span::styled("╔", wall_style)));
    f.render_widget(p, Rect { x: 0, y: 1, width: 1, height: 1 });
    let p = Paragraph::new(Line::from(Span::styled("╗", wall_style)));
    f.render_widget(
        p,
        Rect {
            x: area.width.saturating_sub(1),
            y: 1,
            width: 1,
            height: 1,
        },
    );
    let p = Paragraph::new(Line::from(Span::styled("╚", wall_style)));
    f.render_widget(
        p,
        Rect {
            x: 0,
            y: area.height.saturating_sub(2),
            width: 1,
            height: 1,
        },
    );
    let p = Paragraph::new(Line::from(Span::styled("╝", wall_style)));
    f.render_widget(
        p,
        Rect {
            x: area.width.saturating_sub(1),
            y: area.height.saturating_sub(2),
            width: 1,
            height: 1,
        },
    );
}

fn draw_furniture_piece(f: &mut Frame, furn: &Furniture, area: Rect) {
    let offset_y = 2_u16;
    for (i, line) in furn.art.iter().enumerate() {
        let py = furn.y + offset_y + i as u16;
        let px = furn.x + 1;
        if py < area.height.saturating_sub(2) && px + line.len() as u16 <= area.width {
            let p = Paragraph::new(Line::from(Span::styled(
                *line,
                Style::default().fg(Color::Rgb(120, 120, 160)),
            )));
            f.render_widget(
                p,
                Rect {
                    x: px,
                    y: py,
                    width: line.len() as u16,
                    height: 1,
                },
            );
        }
    }
    let label_y = furn.y + offset_y + furn.art.len() as u16;
    let label_x = furn.x + 1;
    if label_y < area.height.saturating_sub(2) {
        let p = Paragraph::new(Line::from(Span::styled(
            furn.label,
            Style::default().fg(Color::Rgb(80, 80, 110)),
        )));
        f.render_widget(
            p,
            Rect {
                x: label_x,
                y: label_y,
                width: furn.label.len() as u16,
                height: 1,
            },
        );
    }
}

fn draw_plant(f: &mut Frame, plant: &PlantPos, thirst_ratio: f32, area: Rect) {
    let offset_y: u16 = 2;
    let plant_x = plant.screen_x;
    let plant_y = plant.screen_y;
    let art = plant_art(thirst_ratio);

    let art_color = if thirst_ratio < 0.5 {
        Color::Rgb(80, 180, 80)
    } else if thirst_ratio < 0.8 {
        Color::Rgb(200, 170, 60)
    } else {
        Color::Rgb(200, 70, 70)
    };

    for (i, line) in art.iter().enumerate() {
        let py = plant_y + offset_y + i as u16;
        let px = plant_x + 1;
        if py < area.height.saturating_sub(2) && px + line.len() as u16 <= area.width {
            let p = Paragraph::new(Line::from(Span::styled(
                *line,
                Style::default().fg(art_color),
            )));
            f.render_widget(p, Rect { x: px, y: py, width: line.len() as u16, height: 1 });
        }
    }

    let label_y = plant_y + offset_y + art.len() as u16;
    if label_y < area.height.saturating_sub(2) {
        let p = Paragraph::new(Line::from(Span::styled(
            "Plant",
            Style::default().fg(Color::Rgb(80, 80, 110)),
        )));
        f.render_widget(p, Rect { x: plant_x + 1, y: label_y, width: 5, height: 1 });
    }

    // Thirst health bar drawn below the label
    let bar_y = label_y + 1;
    if bar_y < area.height.saturating_sub(2) {
        let bar_width: usize = 8;
        let filled = ((1.0 - thirst_ratio) * bar_width as f32).round() as usize;
        let filled = filled.min(bar_width);
        let bar: String = "\u{2588}".repeat(filled) + &"\u{2591}".repeat(bar_width - filled);
        let bar_color = if thirst_ratio < 0.5 {
            Color::Green
        } else if thirst_ratio < 0.8 {
            Color::Yellow
        } else {
            Color::Red
        };
        let p = Paragraph::new(Line::from(Span::styled(
            bar,
            Style::default().fg(bar_color),
        )));
        f.render_widget(p, Rect { x: plant_x + 1, y: bar_y, width: bar_width as u16, height: 1 });
    }
}

fn draw_character(f: &mut Frame, ch: &Character, area: Rect, selected: bool, tick: usize) {
    let offset_y = 2_u16;
    let cx = (ch.x as u16).clamp(2, area.width.saturating_sub(6));
    let cy = (ch.y as u16 + offset_y).clamp(2, area.height.saturating_sub(6));

    let sprites = ch.sprite_lines();
    let frame_pick = if ch.state == ContainerState::Running {
        (tick / 3) % 2
    } else {
        0
    };

    for (i, (a, b)) in sprites.iter().enumerate() {
        let line_text = if frame_pick == 0 { *a } else { *b };
        let py = cy + i as u16;
        if py < area.height.saturating_sub(2)
            && cx + line_text.len() as u16 <= area.width.saturating_sub(1)
        {
            let style = if selected {
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(ch.color)
            };
            let p = Paragraph::new(Line::from(Span::styled(line_text, style)));
            f.render_widget(
                p,
                Rect {
                    x: cx,
                    y: py,
                    width: line_text.len() as u16,
                    height: 1,
                },
            );
        }
    }

    // Name tag below sprite
    let name_y = cy + sprites.len() as u16;
    if name_y < area.height.saturating_sub(2) {
        let style = if selected {
            Style::default()
                .fg(Color::White)
                .bg(Color::Rgb(60, 60, 100))
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(ch.color).add_modifier(Modifier::DIM)
        };
        let display_name = &ch.name;
        if cx + display_name.len() as u16 <= area.width.saturating_sub(1) {
            let p = Paragraph::new(Line::from(Span::styled(display_name.as_str(), style)));
            f.render_widget(
                p,
                Rect {
                    x: cx,
                    y: name_y,
                    width: display_name.len() as u16,
                    height: 1,
                },
            );
        }
    }

    // Selection / player indicator
    if selected {
        let ind_y = cy.saturating_sub(1);
        if ind_y >= 2 && cx + 3 <= area.width.saturating_sub(1) {
            let p = Paragraph::new(Line::from(Span::styled(
                "★",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
            f.render_widget(p, Rect { x: cx + 1, y: ind_y, width: 1, height: 1 });
        }
    }

    // Thought bubble
    if let Some((ref text, _ttl)) = ch.thought {
        let bubble_y = cy.saturating_sub(2);
        let bubble_x = cx + 3;
        if bubble_y >= 2 && bubble_x + text.len() as u16 + 4 <= area.width.saturating_sub(1) {
            let p = Paragraph::new(Line::from(Span::styled(
                "°",
                Style::default().fg(Color::White),
            )));
            f.render_widget(
                p,
                Rect {
                    x: cx + 2,
                    y: cy.saturating_sub(1),
                    width: 1,
                    height: 1,
                },
            );

            let bubble = format!("💬 {}", text);
            let p = Paragraph::new(Line::from(Span::styled(
                bubble.clone(),
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Rgb(50, 50, 80)),
            )));
            f.render_widget(
                p,
                Rect {
                    x: bubble_x,
                    y: bubble_y,
                    width: bubble.len() as u16,
                    height: 1,
                },
            );
        }
    }
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let running = app
        .characters
        .iter()
        .filter(|c| c.state == ContainerState::Running)
        .count();
    let paused = app
        .characters
        .iter()
        .filter(|c| c.state == ContainerState::Paused)
        .count();
    let exited = app
        .characters
        .iter()
        .filter(|c| c.state == ContainerState::Exited)
        .count();
    let since = app.last_refresh.elapsed().as_secs();

    let selected_name = app
        .characters
        .get(app.selected)
        .map(|c| c.name.as_str())
        .unwrap_or("—");

    let plants_display: String = app
        .plant_thirsts
        .iter()
        .map(|&t| {
            let r = t as f32 / PLANT_MAX_THIRST as f32;
            if t == 0 { "💧" } else if r < 0.5 { "🌿" } else if r < 0.8 { "🥀" } else { "💀" }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let status = format!(
        " 🟢 {} running  🟡 {} paused  🔴 {} exited  │  Plants: {}  │  Selected: {}  │  Refresh: {}s ago ",
        running, paused, exited, plants_display, selected_name, since,
    );
    let y = area.height.saturating_sub(1);
    let p = Paragraph::new(Line::from(Span::styled(
        status,
        Style::default()
            .fg(Color::Rgb(180, 180, 200))
            .bg(Color::Rgb(30, 30, 50)),
    )));
    f.render_widget(p, Rect { x: 0, y, width: area.width, height: 1 });
}
