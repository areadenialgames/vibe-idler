mod app;
mod audio;
mod game;
mod input;
mod save;
mod ui;
mod data;

use std::io;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

fn main() -> io::Result<()> {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        original_hook(panic_info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load saved game or start fresh
    let state = match save::load_game() {
        Ok(Some(saved)) => saved,
        _ => game::state::GameState::new(),
    };
    let audio = audio::AudioHandle::new();
    let mut app = app::App::with_state(state, audio);

    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();
    let mut save_timer = Instant::now();

    loop {
        terminal.draw(|frame| ui::render(frame, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_input(key);
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }

        // Auto-save every 60 seconds
        if save_timer.elapsed() >= Duration::from_secs(60) {
            let _ = save::save_game(&app.state);
            save_timer = Instant::now();
        }

        if !app.running {
            break;
        }
    }

    // Shutdown audio and save on exit
    app.shutdown_audio();
    let _ = save::save_game(&app.state);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
