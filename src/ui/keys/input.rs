use crate::{
  action::{IoSignal, ModeType},
  ui::Tui,
};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::{error::Error, time::Duration};

impl Tui {
  /// Returns `stdin` translated into [`IoSignal`].
  pub fn handle_input(&mut self) -> std::result::Result<IoSignal, Box<dyn Error>> {
    // crossterm::event::read() parks the thread for indefnite time that can cause staleness for immediate mode tuis.
    // The following if clause waits 50ms for an key event to arrive.
    // As the event expires it returns Ok(IoSignal::None)
    //
    // Note: if a event comes within 50ms then the task would execute normally and the poll will retrigger once the current ongoinf loop completes its cycle.
    if !crossterm::event::poll(Duration::from_millis(50))? {
      return Ok(IoSignal::None);
    }

    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
          return Ok(IoSignal::Quit);
        }

        KeyCode::Char('w') if key.modifiers == KeyModifiers::CONTROL => {
          if crossterm::event::poll(Duration::from_millis(500))? {
            if let Event::Key(next) = crossterm::event::read()? {
              match next.code {
                // For vim users Vertical is Horizontal.
                KeyCode::Char('v') => return Ok(IoSignal::Hsplit),
                KeyCode::Char('h') => return Ok(IoSignal::Vsplit),
                _ => {}
              }
            }
          }
        }

        KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
          if crossterm::event::poll(Duration::from_millis(500))? {
            if let Event::Key(next) = crossterm::event::read()? {
              match next.code {
                KeyCode::Char('h') => return Ok(IoSignal::DelVsplit),
                KeyCode::Char('v') => return Ok(IoSignal::DelHsplit),
                _ => {}
              }
            }
          }
        }

        KeyCode::Char('l') | KeyCode::Right if self.editor.mode == ModeType::Normal => {
          return Ok(IoSignal::Right);
        }
        KeyCode::Char('h') | KeyCode::Left if self.editor.mode == ModeType::Normal => {
          return Ok(IoSignal::Left);
        }
        KeyCode::Char('j') | KeyCode::Down if self.editor.mode == ModeType::Normal => {
          return Ok(IoSignal::Down);
        }
        KeyCode::Char('k') | KeyCode::Up if self.editor.mode == ModeType::Normal => {
          return Ok(IoSignal::Up);
        }

        KeyCode::Char('i') => {
          self.editor.mode = ModeType::Insert;
        }

        KeyCode::Char(char) if self.editor.mode == ModeType::Insert => {
          self.editor.insert_char(char);
        }
        KeyCode::Esc => self.editor.mode = ModeType::Normal,

        KeyCode::Backspace => {
          self.editor.remove_char();
        }

        KeyCode::Enter => {
          self.editor.new_line();
        }

        _ => {}
      }
    }
    Ok(IoSignal::None)
  }
}
