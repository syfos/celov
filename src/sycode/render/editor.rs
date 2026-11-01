use crate::sycode::core::Editor;
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture},
  execute,
};
use ratatui::DefaultTerminal;
use std::{error::Error, io::Write, result::Result};

impl Editor {
  /// Result returning wrapper around [`ratatui::run`]. Purely cosmetic.
  pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
    execute!(std::io::stdout(), EnableMouseCapture)?;
    let result = ratatui::run(|terminal| self.renderer(terminal));
    execute!(std::io::stdout(), DisableMouseCapture)?; // runs even if the editor errored
    result
  }

  // Renders the Editor and its components like Fgit, Shell and other visual features using ratatui.
  pub fn renderer(
    &mut self,
    terminal: &mut DefaultTerminal,
  ) -> std::result::Result<(), Box<dyn Error>> {
    loop {
      write!(std::io::stdout(), "\x1b[?1007l")?;
      std::io::stdout().flush()?;

      // To catch Err in the closure
      let mut catch_error = Ok(());

      // The main rendering function for Celov.
      terminal.draw(|frame| {
        // Get frames height and width.
        let area = frame.area();

        // Save for project wide access.
        self.viewport_height = area.height as usize;
        self.viewport_width = area.width as usize;

        // Render Splits.
        self.splits.render(frame);

        // Render the current buffer.
        // Note: terminal.draw() don't allows to return anything except ()
        catch_error = self.display_rope(frame);
        let max_text_row = self
          .softwrap
          .display_lines
          .last_key_value()
          .map(|(_, v)| v.row_range.end)
          .unwrap_or(0);

        // mutate row position and clamp max at max row.
        self.cursor.row_pos = self.cursor.row_pos.min(max_text_row.saturating_sub(2));
      })?;
      catch_error?;

      // Open the error here.
      // Note: If Err then loop will break.

      // If break signal via keys then -> Break.
      // Executes specific task via keys.
      if self.enable_modal_keymaps()? {
        break Ok(());
      }
    }
  }
}
