#[cfg(feature = "tui")]
pub mod app;
#[cfg(feature = "tui")]
pub mod components;
#[cfg(feature = "tui")]
pub mod events;
#[cfg(feature = "tui")]
pub mod layout;
#[cfg(feature = "tui")]
pub mod intro;

pub mod themes;

#[cfg(feature = "tui")]
pub use app::App;

#[cfg(feature = "tui")]
pub fn run_configurator() -> Result<(), Box<dyn std::error::Error>> {
    App::run()
}

#[cfg(feature = "tui")]
pub fn run_intro() -> Result<(), Box<dyn std::error::Error>> {
    use crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};
    use std::io;

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut intro_app = intro::IntroApp::new();

    // Main loop
    let result = loop {
        terminal.draw(|f| intro_app.render(f))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Esc => {
                    intro_app.skip_intro();
                    break Ok(());
                }
                KeyCode::Enter | KeyCode::Right => {
                    intro_app.next_step();
                    if intro_app.should_continue() {
                        // Restore terminal before starting configurator
                        disable_raw_mode()?;
                        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                        terminal.show_cursor()?;
                        
                        // Run configurator
                        return App::run();
                    }
                }
                KeyCode::Left => {
                    intro_app.prev_step();
                }
                _ => {}
            }

            if intro_app.should_exit() {
                break Ok(());
            }
        }
    };

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

#[cfg(not(feature = "tui"))]
pub fn run_configurator() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("TUI feature is not enabled. Please install with --features tui");
    std::process::exit(1);
}

#[cfg(not(feature = "tui"))]
pub fn run_intro() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("TUI feature is not enabled. Please install with --features tui");
    std::process::exit(1);
}
