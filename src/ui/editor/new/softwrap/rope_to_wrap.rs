use std::collections::VecDeque;

use crate::ui::editor::{
  Editor,
  new::{softwrap::SoftWrap, viewport::Viewport},
};

#[allow(dead_code)]
pub struct WrappedRope {
  // The wrapped string.
  pub data: VecDeque<String>,
  /// The rope line idx.
  pub line_idx: usize,
  /// Row range occupied by wrapped line
  pub rows_occupied: std::ops::RangeInclusive<usize>,
  /// The type of line break char at end of string.
  pub line_break_char: LineBreakChar,
}

/// Contains list of all recognized line break charachters that [`Ropey`] uses to mark a string as a [`RopeSlice`.]
/// Note: This is just to map the rendered viewport line to let the caller easiky know what is the only line break character of this line.
#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
#[allow(nonstandard_style)]
pub enum LineBreakChar {
  LineFeed,
  FromFeed,
  CarriageReturn,
  CarriageReturn_LineFeed,
  VerticalTab,
  ParagraphSeperator,
  LineSeperator,
  NextLine,
  None,
}

// Get rope string -> wrap it -> increment row counter to next empty row -> cslculate another line.

#[allow(dead_code)]
impl Editor {
  /// Iterate on the given rope line indices and return wrapped lines for viewport.
  pub fn rope_to_wrap(
    &mut self,
    softwrap: &mut SoftWrap,
    viewport: &Viewport,
  ) -> VecDeque<WrappedRope> {
    // Rope line index counter
    let mut line_idx = self.scroll_offset;
    // Holds wrapped lines of viewport
    let mut viewport_lines = VecDeque::new();
    // Counter that helps to get non overlapping
    // row ranges from Softwrap::get_row_range
    // E.g of range: [1..=5, 6..=9, 10..=15]
    //
    // Note: internally you update this variable incrementally
    // with the field "row_range" of pub struct "SliceRange".
    let mut start_row = 0usize;

    let mut net_row_count = 0usize;

    while net_row_count >= viewport.height {
      // Note: Each rope.line(idx) gives a string with only 1 valid linebreak at end.
      let string = self.rope.line(line_idx).to_string();

      let data = softwrap.wrap(&string, viewport);
      let slice_range = SoftWrap::get_row_range(&data, line_idx, &mut start_row);
      let line_break_char = Self::detect_trailing_linebreak_char(&string);

      // For unique row ranges like [0..=5, 6..=9, 10..=15]
      // it returns the the net count: 5 + 4 + 6 = 15
      net_row_count += &slice_range.row_range.clone().count();

      viewport_lines.push_back(WrappedRope {
        line_idx,
        data,
        line_break_char,
        rows_occupied: slice_range.row_range,
      });

      // increment
      line_idx += 1;
    }

    viewport_lines
  }

  /// Matches the last most line break unicode character and returns one of variant of [`LineBreakChar`].
  fn detect_trailing_linebreak_char(line: &str) -> LineBreakChar {
    if line.ends_with("\r\n") {
      return LineBreakChar::CarriageReturn_LineFeed;
    }

    match line.chars().last() {
      Some('\n') => LineBreakChar::LineFeed,
      Some('\r') => LineBreakChar::CarriageReturn,
      Some('\u{0B}') => LineBreakChar::VerticalTab,
      Some('\u{0C}') => LineBreakChar::FromFeed,
      Some('\u{85}') => LineBreakChar::NextLine,
      Some('\u{2028}') => LineBreakChar::LineSeperator,
      Some('\u{2029}') => LineBreakChar::ParagraphSeperator,
      _ => LineBreakChar::None,
    }
  }
}
