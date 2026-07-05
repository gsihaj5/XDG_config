mod config;
mod cursor;
mod git;
mod model;
mod mount;
mod scanner;
mod ui;

use std::io::stdout;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use config::{default_config_path, load_config};
use model::{App, Modal};

#[derive(Parser)]
#[command(name = "git-sync-tui", about = "Compare and sync git repos across machines")]
struct Cli {
    /// Path to config file
    #[arg(short, long)]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = cli.config.unwrap_or_else(default_config_path);

    if !config_path.exists() {
        eprintln!(
            "Config not found at {}.\nCopy config.example.toml to that path and edit it.",
            config_path.display()
        );
        std::process::exit(1);
    }

    let config = load_config(&config_path).context("load config")?;
    let mut app = App::new(config);

    let result = run_tui(&mut app);
    restore_terminal()?;
    result
}

fn run_tui(app: &mut App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    if handle_key(app, key.code, key.modifiers) {
                        break;
                    }
                }
                Event::Mouse(mouse) => {
                    if matches!(mouse.kind, MouseEventKind::Down(_)) {
                        app.click_at(mouse.column, mouse.row);
                    }
                }
                _ => {}
            }
        }

        if app.should_quit {
            break;
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

fn handle_key(app: &mut App, code: KeyCode, modifiers: KeyModifiers) -> bool {
    // Modal keyboard handling
    match app.modal {
        Modal::SyncConfirm => match code {
            KeyCode::Char('y') | KeyCode::Char('Y') => app.confirm_sync(),
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => app.cancel_modal(),
            _ => {}
        },
        Modal::Result => {
            if matches!(code, KeyCode::Enter | KeyCode::Esc | KeyCode::Char('q')) {
                app.modal = Modal::None;
            }
        }
        Modal::MountPicker => {
            // mount picker is click-only; Esc closes
            if code == KeyCode::Esc {
                app.modal = Modal::None;
            }
        }
        Modal::None => match code {
            KeyCode::Char('q') => return true,
            KeyCode::Char('r') => {
                app.refresh_repos();
                app.status_message = "Refreshed.".to_string();
            }
            KeyCode::Up | KeyCode::Char('k') => app.move_selection(-1),
            KeyCode::Down | KeyCode::Char('j') => app.move_selection(1),
            KeyCode::Enter => app.request_sync(),
            _ => {}
        },
    }

    if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
        return true;
    }

    false
}

fn restore_terminal() -> Result<()> {
    let _ = disable_raw_mode();
    let mut stdout = stdout();
    let _ = execute!(stdout, LeaveAlternateScreen, DisableMouseCapture);
    Ok(())
}
