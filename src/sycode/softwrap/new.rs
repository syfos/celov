use ropey::Rope;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::sycode::unicode::icu_engines::IcuEngines;

pub struct Wrap;
#[allow(dead_code)]
pub struct ProcessedLine {
  pub line_idx: usize,
  pub line_to_byte: usize,
  pub slices: Vec<String>,
}

/// It holds value cumulative cell width of a particular grapheme from [`ProcessedLine`]'s slice along the byte index of that grapheme.
///
/// Note: The byte index is relative to the particular slice which belongs to an element of `slices` of [`ProcessesLine`]
pub struct SliceGraphemeWidth {
  pub byte_idx: usize,
  pub cumulative_width: usize,
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

  fn get_cumulative_width_sums(slice: &str) -> Vec<SliceGraphemeWidth> {
    let mut prev_cumulative_width = 0usize;
    let mut byte_idx = 0usize;
    let mut cumsum_width = Vec::new();

    for grapheme in slice.graphemes(true) {
      byte_idx += grapheme.len();
      prev_cumulative_width += grapheme.width();
      cumsum_width.push(SliceGraphemeWidth {
        byte_idx,
        cumulative_width: prev_cumulative_width,
      });
    }

    cumsum_width
  }
}

