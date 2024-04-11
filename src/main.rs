mod app;
mod cli;
mod cloud_client;
mod errors;
mod logger;
mod tui;
mod utilities;

use std::{error::Error, io};

static APPLICATION_NAME: &str = "csu";

use crate::app::App;
use crate::cloud_client::dropbox::client::DropboxClient;
use crate::logger::setup_logger;
use crate::tui::run_app;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv();

    setup_logger();

    let cloud_client = DropboxClient::build()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(cloud_client);
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
