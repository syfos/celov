use std::ops::Add;

use crate::sycode::cursor::cursor_struct::Cursor;

/*
HashMap<line_idx, Vec<Slice>>
where each Slice is a string,
you call grapheme width of it,
cursor motion respect grapheme,
last index of vector -> has line break,
*/

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
  /// Move cursor `right` by `n` columns.
  /// Note: Clamps at the last most row of viewport.
  pub fn move_right(&mut self, next_grapheme_width: usize, line_slice_width: usize, max_viewport_row: usize) {
    // Move to next row start if there is one and cursor is at width of given slice.
    if self.col_pos.eq(&line_slice_width) && self.row_pos.ne(&max_viewport_row) {
      self.move_down_row(1, max_viewport_row);
      self.col_pos = 0;
      return;
    }

    // else move in current line
    self.col_pos = self.col_pos.add(next_grapheme_width).min(line_slice_width);
  }
}
