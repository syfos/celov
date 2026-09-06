use std::{collections::VecDeque, ops};

use icu_segmenter::{LineSegmenter, LineSegmenterBorrowed, options::LineBreakOptions};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::ui::editor::new::viewport::Viewport;

pub mod rope_to_wrap;

pub struct SoftWrap {
  pub line_segementer: LineSegmenterBorrowed<'static>,
}

impl SoftWrap {
  pub fn new() -> Self {
    Self {
      line_segementer: LineSegmenter::new_auto(LineBreakOptions::default()),
    }
  }
}

pub struct WrappedLine {
  pub rope_line_idx: usize,
  pub string: VecDeque<String>,
  pub rows_occupied: std::ops::RangeInclusive<usize>,
}

/// Holds `cumulative`width of a grapheme of a slice string
/// along the byte index the grapheme belongs to.
pub struct SliceData {
  pub byte_idx: usize,
  pub grapheme_cumulative_width: usize,
}

#[allow(dead_code)]
impl SoftWrap {
  /// Wraps the given `RopeSlice` string into
  /// slices that fit well under `Viewport width.`
  ///
  /// Note: Only the last value of the returned vector will contain a line break char/unicode.
  #[allow(dead_code)]
  pub fn wrap(&mut self, rope_line: &str, viewport: &Viewport) -> VecDeque<String> {
    let breakpoints = self.get_breakpoints(rope_line);
    let mut wrap = VecDeque::new();
    let breakpoint_slices = Self::get_breakpoint_slices(rope_line, &breakpoints);

    let mut current_line = String::new();
    let mut current_width = 0usize;

    for str in breakpoint_slices.iter() {
      let slice_width = str.width_cjk();

      if slice_width > viewport.width {
        if !current_line.is_empty() {
          wrap.push_back(std::mem::take(&mut current_line));
          current_width = 0;
        }
        wrap.extend(Self::break_at_grapheme(str, &viewport.width));
      } else if current_width + slice_width > viewport.width {
        wrap.push_back(std::mem::take(&mut current_line));
        current_line.push_str(str);
        current_width = slice_width;
      } else {
        current_line.push_str(str);
        current_width += slice_width;
      }
    }

    if !current_line.is_empty() {
      wrap.push_back(current_line);
    }

    wrap
  }
}

pub struct SliceRange {
  pub line_idx: usize,
  pub row_range: ops::RangeInclusive<usize>,
}

// Getters defined here
impl SoftWrap {
  /// Returns the inclusive range of rows that have been occupied by the `wrapped line` given line.
  ///
  /// Notes:
  /// 1. The first parameter stands for a single line that have been stored in 1 or more than 1 slice.
  ///
  /// 2. The second parameter stands for rope line index of the given line.
  ///
  /// 3. The third parameter is a counter that must be set to zero.
  pub fn get_row_range(
    wrapped_line: &VecDeque<String>,
    line_idx: usize,
    start_row: &mut usize,
  ) -> SliceRange {
    // assign end row counter
    let end_counter = *start_row + wrapped_line.len().saturating_sub(1);

    // Cache the row range
    let row_range = *start_row..=end_counter;

    // Mutate the start row counter
    *start_row += wrapped_line.len();

    SliceRange {
      line_idx,
      row_range,
    }
  }

  /// Returns the vector containing breakpoints of given string.
  /// Note: the breakpoints are `unicode-aware`, `grapheme-aware` and more specifically `scripto continua-aware`
  fn get_breakpoints(&mut self, rope_line: &str) -> Vec<usize> {
    self.line_segementer.segment_str(rope_line).collect()
  }

  /// Get the slices at valid break points of strings.
  #[allow(dead_code)]
  fn get_breakpoint_slices(rope_line: &str, breakpoints: &[usize]) -> Vec<String> {
    breakpoints
      .windows(2)
      .map(|w| rope_line[w[0]..w[1]].to_string())
      .collect()
  }
  /// Returns the `byte_idx` of the given slice whose `cumulative width` is less than or equal to `viewport`'s width and the most closest to the viewport width.
  ///
  /// E.g:
  /// ```
  /// // Imagine a vector of struct SliceData
  /// let cumulative_widths = [0, 15, 30, 45, 60, 75, 90];
  /// let byte_indicies = [a, b, c, d, e, f, g];
  /// let viewport_width = 55;
  ///
  /// most_equal(&slice_data_vector, viewport_width)
  /// // answer -->
  /// // byte_idx: d (45 is nearmost less than/equal side that is near to 55)
  /// ```
  fn most_equal(cumulative_widths_of_grapheme: &[SliceData], viewport_width: &usize) -> usize {
    // The element index already has byte idx and the second value.
    let matched_element_idx = cumulative_widths_of_grapheme
      .partition_point(|grapheme| grapheme.grapheme_cumulative_width <= *viewport_width)
      .saturating_sub(1);
    cumulative_widths_of_grapheme
      .get(matched_element_idx)
      .unwrap()
      .byte_idx
  }

  /// Returns `cumulative width` data along `byte index` for each
  /// `grapheme` of the given rope string slice.
  fn get_cumulative_widths_of_graphemes(slice: &str) -> Vec<SliceData> {
    let mut cumulative_width_counter_per_grapheme = 0usize;
    let mut byte_idx = 0usize;
    let mut cumulative_widths_of_graphemes = Vec::new();
    for grpaheme in slice.graphemes(true) {
      byte_idx += grpaheme.len();
      cumulative_width_counter_per_grapheme += grpaheme.width_cjk();
      cumulative_widths_of_graphemes.push(SliceData {
        byte_idx,
        grapheme_cumulative_width: cumulative_width_counter_per_grapheme,
      });
    }
    cumulative_widths_of_graphemes
  }

  fn break_at_grapheme(slice: &str, viewport_width: &usize) -> Vec<String> {
    //
    let grapheme_aware_break = Self::most_equal(
      &Self::get_cumulative_widths_of_graphemes(slice),
      viewport_width,
    );

    let mut wrap = Vec::new();
    wrap.push(slice[..grapheme_aware_break].to_string());

    let remainder = &slice[grapheme_aware_break..];
    // loops for all the cases where the remainer
    // would be wider than viewport_width
    // Note: The last value will be always dropped
    if remainder.width_cjk() > *viewport_width {
      wrap.extend(Self::break_at_grapheme(remainder, viewport_width));
    }
    // This will catch such remainder whose terminal
    // width is lesser than viewport width
    else {
      wrap.push(remainder.to_string());
    }

    wrap
  }
}
