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
      let mut render_result = Ok(());

      terminal.draw(|frame| {
        let area = frame.area();
        self.viewport_height = area.height as usize;
        self.viewport_width = area.width as usize;
        self.splits.render(frame);
        render_result = self.render_rope(frame, area);
      })?;

      render_result?;

      if self.enable_modal_keymaps()? {
        break Ok(());
      }
    }
  }
}
