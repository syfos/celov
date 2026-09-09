use crate::{action::ModeType, ui::editor::Editor};

// Note: The keymaps are editor(Sycode) specific, that's why I've scoped the function to Editor.
//
// Note: The Shell and Fgit will have seperate maps from Sycode.
#[allow(dead_code)]
impl Editor {
  pub fn enable_modal_keymaps(&mut self) -> std::io::Result<bool> {
    match self.mode {
      ModeType::Normal => self.normal_mode_keymaps(),
      ModeType::Visual => self.normal_mode_keymaps(),
      ModeType::Insert => self.insert_mode_keymaps(),
    }
  }
}
