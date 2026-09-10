use crossterm::event::{Event, KeyCode};

use std::time::Duration;

use crate::sycode::{core::Editor, keys::editor::ModeType};

impl Editor {
  pub fn insert_mode_keymaps(&mut self) -> std::io::Result<bool> {
    if !crossterm::event::poll(Duration::from_millis(50))? {
      // false means work-done/ending
      return Ok(false);
    }

    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        // Movement Controls
        KeyCode::Up => {
          self.decrement_cursor_row(1);
          return Ok(false);
        }

        KeyCode::Down => {
          self.increment_cursor_row(1);
          return Ok(false);
        }

        KeyCode::Left => {
          self.decrement_cursor_col(1);
          return Ok(false);
        }

        KeyCode::Right => {
          self.increment_cursor_col(1);
          return Ok(false);
        }

        KeyCode::Char(char) => {
          self.insert_char(char);
        }

        KeyCode::Esc => self.mode = ModeType::Normal,

        KeyCode::Backspace => {
          self.remove_char();
        }

        KeyCode::Enter => {
          self.new_line();
        }
        _ => {}
      }
    }

    Ok(false)
  }
}
