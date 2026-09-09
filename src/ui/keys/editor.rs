use crate::{
  action::ModeType,
  ui::{Tui, editor::Editor},
};

// Note: The keymaps are editor(Sycode) specific, that's why I've scoped the function to Editor.
//
// Note: The Shell and Fgit will have seperate maps from Sycode.
#[allow(dead_code)]
impl Editor {
  pub fn enable_modal_keymaps(&mut self) -> std::io::Result<bool> {
    match self.mode {
      ModeType::Normal => self.normal(),
      ModeType::Visual => self.normal(),
      ModeType::Insert => self.normal(),
    }
  }
}
