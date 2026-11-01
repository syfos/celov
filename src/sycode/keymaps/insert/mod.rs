use crate::sycode::{core::Editor, keymaps::modal::ModeType};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::io;

impl Editor {
  pub fn insert_mode_keymaps(&mut self, key: KeyEvent) -> io::Result<()> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
      KeyCode::Esc => {
        self.cursor.move_left(1); // vim steps back when leaving insert
        self.mode = ModeType::Normal;
      }

      // Movement
      KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
        self.move_cursor(key, 1);
      }

      // Typing (Shift+char arrives as an uppercase Char, so only exclude Ctrl)
      KeyCode::Char(_c) if !ctrl => {
        // self.insert_char(_c);
      }

      KeyCode::Enter => {
        // self.new_line();
      }
      KeyCode::Backspace => {
        // self.remove_char();
      }
      KeyCode::Delete => {
        // self.delete_char();
      }
      KeyCode::Tab => {
        // self.insert_char('\t'); or insert spaces
      }

      _ => {}
    }
    Ok(())
  }
}
