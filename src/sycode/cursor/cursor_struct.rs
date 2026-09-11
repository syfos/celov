/// Cordinates of native cursor. 
/// Note: Ratatui accepts cordinates in `(col, row)` format instead of `(row, col)`.
#[derive(Default, Clone, Copy)]
pub struct Cursor {
  /// Postition of cursor in column.
  pub col_pos: usize,
  /// Position of cursor in rows.
  pub row_pos: usize,
  /// The rope line index the cursor is standing on.
  pub current_line_idx: usize,
}
