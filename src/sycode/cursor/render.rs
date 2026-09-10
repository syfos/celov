use std::io::stdout;

use crate::sycode::core::Cursor;
use crate::sycode::keymaps::editor::ModeType;
use crossterm::cursor::SetCursorStyle;
use crossterm::execute;
use ratatui::Frame;

#[allow(dead_code)]
impl Cursor {
  pub fn render(
    &mut self,
    mode: &ModeType,
    frame: &mut Frame,
  ) -> Result<(), Box<dyn std::error::Error>> {
    match mode {
      ModeType::Normal | ModeType::Visual => {
        self.draw(frame, SetCursorStyle::BlinkingBlock)?;
      }

      ModeType::Insert => {
        self.draw(frame, SetCursorStyle::BlinkingBar)?;
      }
    }
    Ok(())
  }

  fn draw(
    &mut self,
    frame: &mut Frame,
    cursor_type: SetCursorStyle,
  ) -> Result<(), Box<dyn std::error::Error>> {
    frame.set_cursor_position((self.0 as u16, self.1 as u16));
    execute!(stdout(), cursor_type)?;
    Ok(())
  }
}
