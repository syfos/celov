use std::collections::BTreeMap;

use ropey::Rope;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::sycode::unicode::icu_engines::IcuEngines;

#[derive(Default)]
pub struct Softwrap {
  // k -> Line idx, v -> Fitting slices/Fitting Line
  pub displayed_lines: BTreeMap<usize, Vec<String>>,
}
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

#[derive(Default)]
pub struct FittingSlices {
  pub line_idx: usize,
  pub slices: Vec<String>,
}

#[allow(dead_code)]
impl Softwrap {
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

  /// Returns nearmost byte index whose cumulative sum is less than or equal to viewport width.
  fn nearmost_byte_idx(cumsum_widths: &[SliceGraphemeWidth], viewport_width: usize) -> usize {
    let matched_element_idx = cumsum_widths
      .partition_point(|grapheme| grapheme.cumulative_width <= viewport_width)
      .saturating_sub(1);

    cumsum_widths.get(matched_element_idx).unwrap().byte_idx
  }

  /// Breaks a single over-long slice (e.g. a word wider than the viewport) into
  /// pieces that each fit within `viewport_width`.
  fn break_at_nearmost_width(
    slice: &str,
    viewport_width: usize,
    fitting_slices: &mut FittingSlices,
    line_idx: usize,
  ) {
    let cumsum_widths = &Self::get_cumulative_width_sums(slice);
    let byte_idx = Self::nearmost_byte_idx(cumsum_widths, viewport_width);

    let wrapped_slice = slice[0..byte_idx].to_string();
    let remainder = slice[byte_idx..].to_string();

    fitting_slices.slices.push(wrapped_slice);
    fitting_slices.line_idx = line_idx;

    // Guard: don't push an empty remainder (fixes the stray "" entries).
    if remainder.is_empty() {
      return;
    }

    if remainder.width() > viewport_width {
      Self::break_at_nearmost_width(&remainder, viewport_width, fitting_slices, line_idx)
    } else {
      fitting_slices.slices.push(remainder);
    }
  }

  pub fn as_wrapped(
    &mut self,
    icu: &IcuEngines,
    rope: &Rope,
    line_idx: usize,
    viewport_width: usize,
    fitting_slices: &mut FittingSlices,
  ) {
    fitting_slices.slices.clear(); // <-- fix: reset per-call state
    fitting_slices.line_idx = line_idx;

    let line = rope.line(line_idx).to_string();
    let line_width = line.width();

    if line_width.le(&viewport_width) {
      self.displayed_lines.insert(line_idx, vec![line]);
      return;
    }

    let processed_line = Self::get_processed_line(icu, rope, line_idx);

    let mut current_line = String::new();
    let mut current_width = 0usize;

    for slice in &processed_line.slices {
      let slice_width = slice.width();

      if slice_width > viewport_width {
        if !current_line.is_empty() {
          fitting_slices
            .slices
            .push(std::mem::take(&mut current_line));
          current_width = 0;
        }
        Self::break_at_nearmost_width(slice, viewport_width, fitting_slices, line_idx);
        continue;
      }

      if current_width + slice_width > viewport_width {
        fitting_slices
          .slices
          .push(std::mem::take(&mut current_line));
        current_width = 0;
      }

      current_line.push_str(slice);
      current_width += slice_width;
    }

    if !current_line.is_empty() {
      fitting_slices.slices.push(current_line);
    }

    self
      .displayed_lines
      .insert(line_idx, fitting_slices.slices.clone());
  }
}

