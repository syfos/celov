use crate::sycode::{core::Editor, keymaps::modal::ModeType};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{io, time::Duration};

const SEQ_TIMEOUT: Duration = Duration::from_millis(500);

impl Editor {
  pub fn normal_mode_keymaps(&mut self, key: KeyEvent) -> io::Result<()> {
    if self.accumulate_count(key) {
      return Ok(());
    }
    let n = self.take_count();

    if self.move_cursor(key, n) {
      return Ok(());
    }

    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    match key.code {
      KeyCode::Esc => {} // count already cleared

      // Mode switches
      KeyCode::Char('i') if !ctrl => self.mode = ModeType::Insert,
      KeyCode::Char('a') if !ctrl => {
        self.cursor.move_right(&self.softwrap, 1, self.viewport_height);
        self.mode = ModeType::Insert;
      }
      KeyCode::Char('v') if !ctrl => self.mode = ModeType::Visual,
      // TODO: 'I', 'A', 'o', 'O' once you have line-start/end/insert-line APIs.

      // Splits (vim users: vertical is horizontal)
      KeyCode::Char('w') if ctrl => {
        if let Some(next) = self.next_key(SEQ_TIMEOUT)? {
          match next.code {
            KeyCode::Char('v') => { /* self.splits.horizontal.increment_count(); */ }
            KeyCode::Char('h') => { /* self.splits.vertical.increment_count(); */ }
            _ => {}
          }
        }
      }
      KeyCode::Char('d') if ctrl => {
        if let Some(next) = self.next_key(SEQ_TIMEOUT)? {
          match next.code {
            KeyCode::Char('h') => { /* vertical decrement + del_split */ }
            KeyCode::Char('v') => { /* horizontal decrement + del_split */ }
            _ => {}
          }
        }
      }

      _ => {}
    }
    Ok(())
  }

  pub fn visual_mode_keymaps(&mut self, key: KeyEvent) -> io::Result<()> {
    if self.accumulate_count(key) {
      return Ok(());
    }
    let n = self.take_count();

    if self.move_cursor(key, n) {
      // TODO: extend selection from anchor to cursor here.
      return Ok(());
    }

    match key.code {
      KeyCode::Esc | KeyCode::Char('v') => self.mode = ModeType::Normal,
      // TODO: 'd', 'y', 'c' acting on the selection.
      _ => {}
    }
    Ok(())
  }
}
