use crate::cli::{Cli, Command, Workspace};
use crate::cloud_client::CloudClient;
use crate::errors::PARSE_COMMAND_ERROR;
use crate::tui::WorkMode;
use tracing::debug;

pub struct App<C: CloudClient> {
    pub input_command: String,
    pub cursor_position: usize,
    pub work_mode: WorkMode,
    pub logs: Vec<String>,
    pub cloud_client: C,
}

impl<C: CloudClient> App<C> {
    pub fn new(cloud_client: C) -> Self {
        Self {
            input_command: String::new(),
            cursor_position: 0,
            work_mode: WorkMode::Read,
            logs: Vec::new(),
            cloud_client,
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input_command.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input_command.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input_command.chars().skip(current_index);

            self.input_command = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input_command.len())
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    pub fn submit_command(&mut self) {
        debug!("Input command: {}", self.input_command);

        self.logs.push(self.input_command.clone());

        match Cli::parse_str(&self.input_command) {
            Ok(cli) => match self.execute_command(cli) {
                Ok(_) => {}
                Err(error) => self.logs.push(error),
            },
            Err(_) => self.logs.push(PARSE_COMMAND_ERROR.to_string()),
        }

        self.input_command.clear();
        self.reset_cursor();
    }

    pub fn execute_command(&self, cli: Cli) -> Result<(), String> {
        match cli.command {
            Command::Download { from_path, to_path } => {
                debug!("Downloading... {:?} {:?}", from_path, to_path);
                self.cloud_client.download(from_path, to_path)?;
            }
            Command::Upload { from_path, to_path } => {
                debug!("Uploading... {:?} {:?}", from_path, to_path);
                self.cloud_client.upload(from_path, to_path)?;
            }
            Command::Delete { path } => {
                debug!("Deleting... {:?}", path);
                self.cloud_client.delete(path)?;
            }
            Command::List { workspace } => match workspace {
                Workspace::Local => {
                    debug!("Listing local entries...")
                }
                Workspace::Cloud => {
                    debug!("Listing cloud entries...")
                }
            },
        }
        Ok(())
    }
}
