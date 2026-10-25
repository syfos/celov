use ratatui::DefaultTerminal;
use std::{error::Error, result::Result};

use crate::sycode::core::Editor;

impl Editor {
  /// Wrapper over [`ratatui::run`].
  pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
    ratatui::run(|terminal| self.renderer(terminal))?;
    Ok(())
  }

  /// Main renderer that renders Fgit's whole Tui.
  pub fn renderer(
    &mut self,
    terminal: &mut DefaultTerminal,
  ) -> std::result::Result<(), Box<dyn Error>> {
    loop {
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
        catch_error = self.render_rope(frame, area);
      })?;

      // Open the error here.
      // Note: If Err then loop will break.
      catch_error?;

      // If break signal via keys then -> Break.
      // Executes specific task via keys.
      if self.enable_modal_keymaps()? {
        break Ok(());
      }
    }
  }
}
