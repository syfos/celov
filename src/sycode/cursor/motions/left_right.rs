use crate::sycode::{cursor::cursor_struct::Cursor, softwrap::word_wrap::Softwrap};

#[allow(dead_code)]
impl Cursor {
  pub fn move_left(&mut self, softwrap: &Softwrap, target: usize, viewport_height: usize) {
    self.col_pos = self.col_pos.saturating_sub(1);
    let target_data = self.get_target_data(softwrap, target, viewport_height);
  }
}
