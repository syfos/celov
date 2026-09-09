use ratatui::DefaultTerminal;
use std::{error::Error, result::Result};

use crate::ui::editor::new::core::Editor;

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
      terminal.draw(|frame| {
        let area = frame.area();
        self.viewport_height = area.height as usize;
        self.viewport_width = area.width as usize;
        self.splits.render(frame);
        self.render_rope(frame, area);
      })?;

      if self.enable_modal_keymaps()? {
        break Ok(());
      }
    }
  }
}
