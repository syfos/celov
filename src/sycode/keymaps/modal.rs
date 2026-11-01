use crate::sycode::core::Editor;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::{io, time::Duration};

#[derive(Default, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum ModeType {
  #[default]
  Normal,
  Visual,
  Insert,
}

impl Editor {
  /// Ok(true) = quit, Ok(false) = keep looping.
  pub fn enable_modal_keymaps(&mut self) -> io::Result<bool> {
    if !event::poll(Duration::from_millis(50))? {
      return Ok(false);
    }

    let key = match event::read()? {
      Event::Key(k) if k.kind == KeyEventKind::Press => k,
      _ => return Ok(false), // mouse, resize, focus, paste: swallowed
    };

    // Global binding, works in every mode.
    if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
      return Ok(true);
    }

    match self.mode {
      ModeType::Normal => self.normal_mode_keymaps(key)?,
      ModeType::Visual => self.visual_mode_keymaps(key)?,
      ModeType::Insert => self.insert_mode_keymaps(key)?,
    }
    Ok(false)
  }

  /// Wait briefly for a follow-up key press (for Ctrl+W / Ctrl+D sequences).
  pub(crate) fn next_key(&self, timeout: Duration) -> io::Result<Option<KeyEvent>> {
    if event::poll(timeout)? {
      if let Event::Key(k) = event::read()? {
        if k.kind == KeyEventKind::Press {
          return Ok(Some(k));
        }
      }
    }
    Ok(None)
  }

  /// Shared by Normal and Visual. Returns true if the key was a motion.
  pub(crate) fn move_cursor(&mut self, key: KeyEvent, n: usize) -> bool {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
      return false;
    }
    match key.code {
      KeyCode::Char('k') | KeyCode::Up => {
        self
          .cursor
          .move_up_in_rows_by(&self.softwrap, n, self.viewport_height)
      }
      KeyCode::Char('j') | KeyCode::Down => {
        self
          .cursor
          .move_down_in_rows_by(&self.softwrap, n, self.viewport_height)
      }
      KeyCode::Char('h') | KeyCode::Left => self.cursor.move_left(n),
      KeyCode::Char('l') | KeyCode::Right => {
        self
          .cursor
          .move_right(&self.softwrap, n, self.viewport_height)
      }
      // TODO: '0', '$', 'w', 'b', 'gg', 'G' once your cursor API has them.
      _ => return false,
    }
    true
  }

  /// Digits build a count; returns true if the key was consumed as one.
  pub(crate) fn accumulate_count(&mut self, key: KeyEvent) -> bool {
    if let KeyCode::Char(c) = key.code {
      if key.modifiers.is_empty() && c.is_ascii_digit() && (c != '0' || self.count > 0) {
        self.count = self
          .count
          .saturating_mul(10)
          .saturating_add(c as usize - '0' as usize);
        return true;
      }
    }
    false
  }

  pub(crate) fn take_count(&mut self) -> usize {
    std::mem::take(&mut self.count).max(1)
  }
}
