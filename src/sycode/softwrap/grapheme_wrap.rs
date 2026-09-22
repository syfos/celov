use std::ops;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

use crate::sycode::softwrap::word_wrap::Softwrap;

/// Contains grapheme aware break related data for the concerned grapheme of the concerned string.
pub struct GraphemeData {
  /// Cumulative sum of the grapheme's cell width.
  cumsum_width: usize,
  /// The byte index boundary range(Inclusive) of the grapheme.
  byte_idx_range: std::ops::Range<usize>,
}

impl Softwrap {
  pub fn get_overflow_word_string(
    overflowed_line: &str,
    words: &[(usize, ops::Range<usize>)],
  ) -> String {
    let overflowed_word_end = words.first().unwrap().1.end;
    overflowed_line[..overflowed_word_end].to_string()
  }
  /// Returns [`GraphemeData`] conatining byte index boundary and cumulative width of each grapheme.
  fn get_graphemes(line: &str) -> Vec<GraphemeData> {
    let mut grapheme_data = Vec::new();
    let mut cumsum_width_counter = 0;

    for (byte_idx_start, grapheme) in line.grapheme_indices(true) {
      // Track the cumsum width.
      cumsum_width_counter += grapheme.width();

      // .len() for any &str returns the exclusive byte idx where it ends.
      // The below arithimetic is -> prev byte idx offset + byte idx of grapheme
      let byte_idx_exclusive_end = byte_idx_start + grapheme.len();

      // The exclusive range of the grapheme.
      let range = byte_idx_start..byte_idx_exclusive_end;

      grapheme_data.push(GraphemeData {
        cumsum_width: cumsum_width_counter,
        byte_idx_range: range,
      });
    }

    grapheme_data
  }

  /// Returns the nearmost byte index for the line string that fits well into the row of given viewport's width.
  fn get_nearmost_grapheme_byte_idx(line: &str, viewport_width: usize) -> usize {
    let grapheme_data = Self::get_graphemes(line);
    let data_idx = grapheme_data.partition_point(|data| data.cumsum_width <= viewport_width);

    grapheme_data
      .get(data_idx.saturating_sub(1))
      .unwrap()
      .byte_idx_range
      .end
  }

  /// Wraps a rope line/string slice that can't be wrapped using word level wrap.
  /// Note: Returns `None` if string is empty
  pub fn wrap_grapheme_level(line: &str, viewport_width: usize) -> Vec<String> {
    let mut wrapped_line = Vec::new();
    // Return early as the line purely fits in.
    if line.is_empty() {
      return wrapped_line;
    }

    // get relative byte index to the string where break will happen.
    let break_at = Self::get_nearmost_grapheme_byte_idx(line, viewport_width);

    // get valid wrap slice and push it
    let wrapped_slice = &line[..break_at];
    wrapped_line.push(wrapped_slice.into());

    // take the remainder out.
    let remainder = &line[break_at..];

    // If remainder has a width lt or eq to viewport width then it fits in and nothing is left to work on.
    if remainder.width() <= viewport_width {
      wrapped_line.push(remainder.into());
      return wrapped_line;
    }

    // Triggers only where remainder's width is gt viewport width. Will loop until the remainder breaks in desired width.
    wrapped_line.extend(Self::wrap_grapheme_level(remainder, viewport_width));
    wrapped_line
  }
}
