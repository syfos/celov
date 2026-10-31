use unicode_width::UnicodeWidthStr;

use crate::sycode::{cursor::cursor_struct::Cursor, softwrap::word_wrap::Softwrap};

#[allow(dead_code)]
impl Cursor {
  pub fn move_left(&mut self, target: usize) {
    self.col_pos = self.col_pos.saturating_sub(target);
  }

  pub fn move_right(&mut self, softwrap: &Softwrap, target: usize, viewport_height: usize) {
    let target_data = self.get_target_data(softwrap, self.row_pos, viewport_height);
    self.col_pos = self
      .col_pos
      .saturating_add(target)
      .min(target_data.string.width().saturating_sub(1));
  }
}
