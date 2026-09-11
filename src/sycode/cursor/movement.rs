use std::ops::Add;

use crate::sycode::cursor::cursor_struct::Cursor;

// Cursor Slice and Cursor Line
#[allow(dead_code)]
impl Cursor {
  /// Move cursor down by `n` rows.
  /// Note: Clamps at the `max` height of viewport.
  pub fn move_down_row(&mut self, n: usize, max_viewport_row: usize) {
    self.row_pos = self.row_pos.add(n).min(max_viewport_row);
  }

  /// Move cursor `upward` by `n` rows.
  /// Note: Clamps at the first row of viewport.
  pub fn move_up_row(&mut self, n: usize) {
    self.row_pos = self.row_pos.saturating_sub(n);
  }

  /// Move cursor `upward` by `n` rows.
  /// Note: Clamps at the first row of viewport.
  pub fn move_left(&mut self, n: usize) {
    self.col_pos = self.col_pos.saturating_sub(n);
    if self.col_pos.eq(&0usize) && self.row_pos.ne(&0usize) {
      // Todo: Set cursor before line break char of previous line.
    }
  }
}
