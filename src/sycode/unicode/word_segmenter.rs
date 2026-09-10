use std::ops::{self, Add};

use crate::sycode::unicode::{icu_engines::IcuEngines, unicode_struct::Unicode};

// A note about what to do next:
// I was thinking about using grapheme width and word with for wordotion.
// For that I need get the word based cumulative width range
// I don't know how but somehow I need to map cursor with grapheme cluster and its width aware motion.
//
// Say "Hello, World" has the following word byte ranges: [0..5, 5..6, 6..7, 7..12]
//

#[allow(dead_code)]
impl Unicode {
  pub fn get_word_segments(
    cursor_rope_line: &str,
    line_to_byte: usize,
    icu: &IcuEngines,
  ) -> Vec<ops::Range<usize>> {
    // get word segments for cursor line.
    let bounds: Vec<usize> = icu.word.segment_str(cursor_rope_line).collect();

    // Returns absolute byte index range for words.
    bounds
      .windows(2)
      .map(|byte| {
        // Relative byte index
        let (rel_start, rel_end) = (byte[0], byte[1]);

        // Absolute byte index
        rel_start.add(line_to_byte)..rel_end.add(line_to_byte)
      })
      .collect()
  }
}
