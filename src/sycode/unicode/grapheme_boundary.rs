use std::ops;
use unicode_width::UnicodeWidthStr;

use crate::sycode::unicode::{icu_engines::IcuEngines, unicode_struct::Unicode};

#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub struct GraphemeBoundary {
  pub line_idx: usize,

  /// Tells width of current grapheme boundary in terms of `viewport's cells`.
  /// Helps in movement of cursor.
  /// ```
  /// // Usage:
  /// let new_col_pos = cursor_col + next_grapheme.width;
  /// let new_col_pos = cursor_col - prev_grapheme.width;
  /// ```
  pub width: usize,

  /// The cumulative width sum for the current grapheme boundary of the rope string.
  /// Why exists?: To help [`Softwrap`] do the wrap math.
  pub cumulative_width: usize,

  /// The rope byte idx range the graphemes stands on.
  /// E.g, say for a CJK string: [0..3, 3..6, 6..9, 9..12]
  /// Your byte boundaries -> [0..=2, 3..=5, 6..=8, 9..=11]
  pub absolute_byte_idx: ops::Range<usize>,
}
#[allow(dead_code)]
impl Unicode {
  /// This function returns [`GraphemeBoundary`] containing useful data related to the rope line.
  ///
  /// Quick overview:
  /// 1. Line index
  /// 2.  Boundary width in viewport cells, for cursor movement.
  /// 3. Cumulative width of boundary, for [`Softwrap`] based softwrap.
  /// 4. Absolute byte index range of the grapheme in the given rope line.
  ///
  /// Why exists?:
  /// Exists because you can't reliably edit/wrap a rope string without knowing the boundaries.
  pub fn get_grapheme_boundary(
    rope_line: &str,
    line_idx: usize,
    line_to_byte: usize,
    segmenter: IcuEngines,
  ) -> Vec<GraphemeBoundary> {
    // Generate vector containing byte indicies.
    let breakpoints: Vec<usize> = segmenter.grapheme_cluster.segment_str(rope_line).collect();

    let mut cumulative_width = 0usize;

    // Returns exclusive absolute byte index vector.
    // For e.g: [0..3, 3..4, 4..6]
    //
    // Note: Since this default range excludes the end value, you just use it as:
    // 0..=2, 3..=3, 4..=5 with respective counts as 3, 1, and 2
    breakpoints
      // Yeilds the slices/windows each of length 2.
      // E.g: [0, 3, 6, 9, 12]
      // 0..3, 3..6, 6..9, 9..12
      .windows(2)
      // byte_window is a slice/window of length 2
      // line_to_byte + byte_idx -> generates absolute byte index for rope level operation.
      .map(|byte_window| {
        // Get the relative start and end byte index
        // Why?: To get the grapheme slice.
        let (rel_start, rel_end) = (byte_window[0], byte_window[1]);

        // Todo: Do check is range exclusive even the right choice.
        //
        // Gives the grapheme slice from the rope string.
        let grapheme_slice = &rope_line[rel_start..rel_end];

        // Get width of current grapheme.
        let width = grapheme_slice.width();

        // Get the cumulative width
        // Why?: For grapheme aware soft wrapping.
        cumulative_width += width;

        // Get the absolute byte index.
        // Why?: To work with rope's insert/remove api.
        let absolute_byte_idx = (line_to_byte + byte_window[0])..(line_to_byte + byte_window[1]);

        // Return
        GraphemeBoundary {
          line_idx,
          width,
          cumulative_width,
          absolute_byte_idx,
        }
      })
      // collect and cast
      .collect()
  }
}
