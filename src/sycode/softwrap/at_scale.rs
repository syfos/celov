use std::ops;

use ropey::Rope;

use crate::sycode::{
  softwrap::word_wrap::{Softwrap, WrappedRope},
  unicode::icu_engines::IcuEngines,
};

#[allow(dead_code)]
impl Softwrap {
  fn into_wrapped(
    &mut self,
    rope: &mut Rope,
    rope_line_indices: ops::RangeInclusive<usize>,
    icu: &IcuEngines,
    viewport_width: usize,
  ) {
    for line_idx in rope_line_indices {
      let rope_string = rope.line(line_idx).to_string();
      let mut wrappings = WrappedRope::default();
      self.wrap(icu, &rope_string, viewport_width, &mut wrappings);
      self.map.insert(line_idx, wrappings);
    }
  }
}
