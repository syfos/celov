use std::io::stdout;

use crate::sycode::core::Cursor;
use crate::sycode::keymaps::editor::ModeType;
use crossterm::cursor::SetCursorStyle;
use crossterm::execute;
use ratatui::Frame;

#[allow(dead_code)]
impl Cursor {
  /// Renders native terminal cursor respecting the [`ModeType`].
  /// Put `is_blinking: false` to stop blinking.
  ///
  /// Note:
  /// 1. Block for `ModeType::Normal` and `ModeType::Visual`.
  /// 2. Bar for `ModeType::Insert`.
  pub fn render(
    &mut self,
    mode: &ModeType,
    frame: &mut Frame,
    is_blinking: bool,
  ) -> Result<(), Box<dyn std::error::Error>> {
    if is_blinking {
      match mode {
        // Normal and Visual need Block cursor.
        ModeType::Normal | ModeType::Visual => {
          self.draw(frame, SetCursorStyle::BlinkingBlock)?;
        }

        // Insert needs Bar cursor.
        ModeType::Insert => {
          self.draw(frame, SetCursorStyle::BlinkingBar)?;
        }
      }
      return Ok(());
    }

    match mode {
      // Normal and Visual need Block cursor.
      ModeType::Normal | ModeType::Visual => {
        self.draw(frame, SetCursorStyle::BlinkingBlock)?;
      }

      // Insert needs Bar cursor.
      ModeType::Insert => {
        self.draw(frame, SetCursorStyle::BlinkingBar)?;
      }
    }
    Ok(())
  }

  /// Draws terminal's native cursor according to the given style.
  fn draw(
    &mut self,
    frame: &mut Frame,
    cursor_type: SetCursorStyle,
  ) -> Result<(), Box<dyn std::error::Error>> {
    // Sets the cursor's postion irrespective of buffer.
    frame.set_cursor_position((self.0 as u16, self.1 as u16));
    // send it to the terminal to process
    execute!(stdout(), cursor_type)?;
    Ok(())
  }
}
