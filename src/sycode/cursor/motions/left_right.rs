use unicode_width::UnicodeWidthStr;

use crate::sycode::{cursor::cursor_struct::Cursor, softwrap::word_wrap::Softwrap};

#[allow(dead_code)]
impl Cursor {
  pub fn move_left(&mut self, softwrap: &Softwrap, target: usize, viewport_height: usize) {
    self.col_pos = self.col_pos.saturating_sub(1);
    let target_data = self.get_target_data(softwrap, target, viewport_height);

    // Just a feature to go back 
    // Purely for testing
    if self.col_pos == 0 && self.row_pos != 0 {
      self.col_pos = target_data.string.width().saturating_add(1);
    }
  }
}
