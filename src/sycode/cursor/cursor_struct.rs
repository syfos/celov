/// Cordinates of native cursor. 
/// Note: Ratatui accepts cordinates in `(col, row)` format instead of `(row, col)`.
#[derive(Default, Clone, Copy)]
pub struct Cursor {
  pub col: usize,
  pub row: usize,
}
