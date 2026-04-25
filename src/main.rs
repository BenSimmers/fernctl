mod modules;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io,
    time::Duration,
};

use modules::app::App;
use modules::draw::draw;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let size = terminal.size()?;
    app.init_plants(size.width, size.height);
    let tick_rate = Duration::from_millis(80);

    terminal.clear()?;

    loop {

        terminal.draw(|f| draw(f, &app))?;

        if crossterm::event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') => app.refresh(),
                    // Tab / Shift+Tab cycle the controlled container
                    KeyCode::Tab => {
                        if !app.characters.is_empty() {
                            app.selected = (app.selected + 1) % app.characters.len();
                        }
                    }
                    KeyCode::BackTab => {
                        if !app.characters.is_empty() {
                            if app.selected == 0 {
                                app.selected = app.characters.len() - 1;
                            } else {
                                app.selected -= 1;
                            }
                        }
                    }
                    // Arrow keys move the controlled container
                    KeyCode::Up    => app.move_selected(0.0, -1.0),
                    KeyCode::Down  => app.move_selected(0.0,  1.0),
                    KeyCode::Left  => app.move_selected(-1.0, 0.0),
                    KeyCode::Right => app.move_selected( 1.0, 0.0),
                    // Space waters the plant when close enough
                    KeyCode::Char(' ') => app.try_water_plant(),
                    _ => {}
                }
            }
        }

        app.update();

        // Auto-refresh every 10 seconds
        if app.last_refresh.elapsed() > Duration::from_secs(10) {
            app.refresh();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
