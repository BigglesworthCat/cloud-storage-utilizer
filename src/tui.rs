use crate::app::App;
use crate::cloud_client::CloudClient;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, Paragraph},
};
use std::io;

pub enum WorkMode {
    Read,
    Edit,
}

pub fn ui<C: CloudClient>(frame: &mut Frame, app: &App<C>) {
    let main_layout = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ]);
    let [help_area, input_area, log_area, storages_area] = main_layout.areas(frame.size());

    let (msg, style) = match app.work_mode {
        WorkMode::Read => (
            vec![
                "Press ".into(),
                "q".bold(),
                " to exit, ".into(),
                "e".bold(),
                " to start editing.".bold(),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        WorkMode::Edit => (
            vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to record the message".into(),
            ],
            Style::default(),
        ),
    };
    let text = Text::from(Line::from(msg)).patch_style(style);
    let help_message = Paragraph::new(text);
    frame.render_widget(help_message, help_area);

    let input = Paragraph::new(app.input_command.as_str())
        .style(match app.work_mode {
            WorkMode::Read => Style::default().fg(Color::LightYellow),
            WorkMode::Edit => Style::default().fg(Color::LightGreen),
        })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input command"),
        );
    frame.render_widget(input, input_area);
    match app.work_mode {
        WorkMode::Read => {}
        WorkMode::Edit => {
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor(
                input_area.x + app.cursor_position as u16 + 1,
                input_area.y + 1,
            );
        }
    }

    let logs = List::new(app.logs.iter().cloned())
        .block(Block::default().borders(Borders::ALL).title("Log"));
    frame.render_widget(logs, log_area);

    let info_layout = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(storages_area);

    let empty_list: Vec<String> = vec![];
    let local_entries = List::new(empty_list.iter().cloned())
        .block(Block::default().borders(Borders::ALL).title("Local files"));
    frame.render_widget(local_entries, info_layout[0]);

    let cloud_entries = List::new(empty_list.iter().cloned())
        .block(Block::default().borders(Borders::ALL).title("Cloud files"));
    frame.render_widget(cloud_entries, info_layout[1]);
}

pub fn run_app<B: Backend, C: CloudClient>(
    terminal: &mut Terminal<B>,
    mut app: App<C>,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.work_mode {
                WorkMode::Read => match key.code {
                    KeyCode::Char('e') => {
                        app.work_mode = WorkMode::Edit;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                WorkMode::Edit if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => app.submit_command(),
                    KeyCode::Char(to_insert) => {
                        app.enter_char(to_insert);
                    }
                    KeyCode::Backspace => {
                        app.delete_char();
                    }
                    KeyCode::Left => {
                        app.move_cursor_left();
                    }
                    KeyCode::Right => {
                        app.move_cursor_right();
                    }
                    KeyCode::Esc => {
                        app.work_mode = WorkMode::Read;
                    }
                    _ => {}
                },
                WorkMode::Edit => {}
            }
        }
    }
}
