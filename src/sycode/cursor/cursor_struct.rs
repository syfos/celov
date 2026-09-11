/// The cursor position in `(col, row)` format.
#[derive(Default, Clone, Copy)]
pub struct Cursor(pub usize, pub usize);

/// This temporary data sturct is ment to reduce chaos ans allow programming.
/// Note: Ratatui accepts (col, row) instead of (row, col).
pub struct CursorNew{
  pub col: usize,
  pub row: usize,
}
