use crate::sycode::{core::Editor, keys::editor::ModeType};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use std::time::Duration;

// True means break
// False means continue the loop
#[allow(dead_code)]
#[allow(clippy::collapsible_if)]
impl Editor {
  pub fn normal_mode_keymaps(&mut self) -> std::io::Result<bool> {
    if !crossterm::event::poll(Duration::from_millis(50))? {
      // false means work-done/ending
      return Ok(false);
    }

    if let Event::Key(key) = crossterm::event::read()? {
      match key.code {
        KeyCode::Char('q') if key.modifiers == KeyModifiers::CONTROL => {
          // true means break the loop
          return Ok(true);
        }

        KeyCode::Char('i') => {
          self.mode = ModeType::Insert;
          return Ok(false);
        }

        KeyCode::Char('w') if key.modifiers == KeyModifiers::CONTROL => {
          if crossterm::event::poll(Duration::from_millis(500))? {
            if let Event::Key(next) = crossterm::event::read()? {
              match next.code {
                // For vim users Vertical is Horizontal.
                KeyCode::Char('v') => {
                  // self.splits.horizontal.increment_count();
                  return Ok(false);
                }
                KeyCode::Char('h') => {
                  // self.splits.vertical.increment_count();
                  return Ok(false);
                }
                _ => {}
              }
            }
          }
        }

        // Todo: Scope split feature to Editor.
        KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
          if crossterm::event::poll(Duration::from_millis(500))? {
            if let Event::Key(next) = crossterm::event::read()? {
              match next.code {
                KeyCode::Char('h') => {
                  // self.splits.vertical.decrement_count();
                  // self.splits.vertical.del_split();
                  return Ok(false);
                }
                KeyCode::Char('v') => {
                  // self.splits.horizontal.decrement_count();
                  // self.splits.horizontal.del_split();
                  return Ok(false);
                }
                _ => {}
              }
            }
          }
        }

        // Movement Controls
        KeyCode::Char('k') | KeyCode::Up => {
          self.decrement_cursor_row(1);
          return Ok(false);
        }

        KeyCode::Char('j') | KeyCode::Down => {
          self.increment_cursor_row(1);
          return Ok(false);
        }

        KeyCode::Char('h') | KeyCode::Left => {
          self.decrement_cursor_col(1);
          return Ok(false);
        }

        KeyCode::Char('l') | KeyCode::Right => {
          self.increment_cursor_col(1);
          return Ok(false);
        }

        _ => {}
      }
    }

    // This handles all unmatched cases.
    // Reaching this means the loop must continue.
    Ok(false)
  }
}
