use crate::ui::{editor::Editor, splits::Splits};
use ratatui::layout::Rect;

pub mod buffer;
pub mod editor;
pub mod keys;
pub mod renderer;
pub mod splits;
pub mod unicode;

/// Tui module of Fgit.
pub struct Tui {
  pub screen_area: Rect,
  pub splits: Splits,
  pub editor: Editor,
}

impl Tui {
  pub fn new() -> std::io::Result<Self> {
    Ok(Self {
      screen_area: Rect::default(),
      splits: Splits::default(),
      editor: Editor::new()?,
    })
  }
}
