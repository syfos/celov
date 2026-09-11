pub struct Wrap;
#[allow(dead_code)]
pub struct ProcessedLine {
  pub line_idx: usize,
  pub line_to_byte: usize,
  pub slices: Vec<String>,
}
