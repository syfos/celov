use std::ops;

use ropey::Rope;

use crate::sycode::{
  softwrap::word_wrap::{Softwrap, WrappedRope},
  unicode::icu_engines::IcuEngines,
};

#[allow(dead_code)]
impl Softwrap {
  /// Converts viewport's unwrapped lines into wrapped lines.
  fn wrap_viewport_lines(
    &mut self,
    rope: &mut Rope,
    rope_line_indices: ops::RangeInclusive<usize>,
    icu: &IcuEngines,
    viewport_width: usize,
  ) {
    let mut start = 0;

    for line_idx in rope_line_indices {
      // Get rope string
      let rope_string = rope.line(line_idx).to_string();

      // We generate WrappesRope each time because
      // Each rope contains its:
      // 1. Wrapped slices
      // 2. Row range to locate on viewport
      let mut wrappings = WrappedRope::default();

      // This mutates the "wrappings" struct.
      self.wrap(icu, &rope_string, viewport_width, &mut wrappings);

      // Each line index covers at least one row.
      // The row range is 0 indexed and hence matches well with rows of viewport.
      //
      // Start = 0, end = 3 => 0..3 -> 0, 1, 2
      // start = 3, end = 5 => 3..8 -> 3, 4, 5, 6, 7
      let end = start + wrappings.slices.len();
      wrappings.row_range = start..end;
      start = end;

      // Push the "wrappings" along its line index into a bTreeMap resembling the viewport.
      self.map.insert(line_idx, wrappings);
    }
  }
}
