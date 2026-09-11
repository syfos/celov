use ropey::Rope;

use crate::sycode::unicode::icu_engines::IcuEngines;

pub struct Wrap;
#[allow(dead_code)]
pub struct ProcessedLine {
  pub line_idx: usize,
  pub line_to_byte: usize,
  pub slices: Vec<String>,
}

#[allow(dead_code)]
impl Wrap {
  /// Returns [`ProcessedLine`] containing `line_idx`, `line_to_byte` of the rope line and its `slices` which are `grapheme` and `scripto continua` aware.
  fn get_processed_line(icu: &IcuEngines, rope: &Rope, line_idx: usize) -> ProcessedLine {
    let line = rope.line(line_idx).to_string();
    let line_to_byte = rope.line_to_byte(line_idx);
    let breakpoints: Vec<usize> = icu.line.segment_str(&line).collect();

    let mut slices = Vec::new();

    for window in breakpoints.windows(2) {
      let slice = line[window[0]..window[1]].to_string();
      slices.push(slice);
    }

    ProcessedLine {
      line_idx,
      line_to_byte,
      slices,
    }
  }
}
