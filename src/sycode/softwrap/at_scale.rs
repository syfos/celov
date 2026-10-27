use ropey::Rope;
use crate::sycode::{
  softwrap::word_wrap::{Softwrap, RopeSlices},
  unicode::icu_engines::IcuEngines,
};

#[allow(dead_code)]
impl Softwrap {
  /// Converts viewport's unwrapped lines into wrapped lines.
  pub fn wrap_viewport_lines(
    &mut self,
    rope: &mut Rope,
    seed_line_idx: &mut usize,
    icu: &IcuEngines,
    viewport_width: usize,
    viewport_height: usize,
  ) {
    let mut start = 0;
    let mut last_row = 0;

    while last_row <= viewport_height  && *seed_line_idx <= rope.len_lines().saturating_sub(1){
      // Get rope string
      let rope_string = rope.line(*seed_line_idx).to_string();

      // We generate WrappesRope each time because
      // Each rope contains its:
      // 1. Wrapped slices
      // 2. Row range to locate on viewport
      let mut wrappings = RopeSlices::default();

      // This mutates the "wrappings" struct.
      self.wrap(icu, &rope_string, viewport_width, &mut wrappings);

      // Each line index covers at least one row.
      // The row range is 0 indexed and hence matches well with rows of viewport.
      //
      // Start = 0, end = 3 => 0..3 -> 0, 1, 2
      // start = 3, end = 5 => 3..8 -> 3, 4, 5, 6, 7
      let end = start + wrappings.slices.len();
      wrappings.row_range = start..end;
      last_row = end;
      start = end;

      // Push the "wrappings" along its line index into a bTreeMap resembling the viewport.
      self.display_lines.insert(*seed_line_idx, wrappings);

      *seed_line_idx += 1;
    }
  }
}
