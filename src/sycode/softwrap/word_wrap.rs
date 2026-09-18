use crate::sycode::unicode::icu_engines::IcuEngines;
use std::{collections::BTreeMap, ops};
use unicode_width::UnicodeWidthStr;

/// Rope lines of the viewport that have been wrapped for word aware visual display.
/// Info:
/// 1. The word aware wrap is scripto continua and scripto distincta.
/// 2. The Grapheme aware wrap is just grapheme cluster aware.
/// Note: Some wrapped slices of the [`WrappedRope`] may contain grapheme cluster aware break instead of word break if there would have been no valid word of width less than or equal to viewport width.
pub struct WordWrap {
  // Key -> Line idx of rope line.
  // Value -> [`WrappedRope`].
  lines: BTreeMap<usize, WrappedRope>,
}

/// Wrapped slices of rope line that take exactly one row.
/// Tip: The length of `wrapped_slices: Vec<String>` == number of rows occupied by the whole rope line.
/// Note: Some wrapped slices of the rope may contain grapheme cluster aware break instead of word aware break if there would have been no valid word of width less than or equal to viewport width.
pub struct WrappedRope {
  wrapped_slices: Vec<String>,
  row_occupied_real_range: ops::Range<usize>,
}

enum FitType {
  /// Only a slice of string can fit in the given viewport width.
  Slice(usize, usize),
  /// The whole string fits in the given viewport width.
  Whole,
  /// No word of string fits for the viewport width.
  /// Do grapheme aware break.
  Overflow,
  /// Words vector is empty.
  Empty,
}

impl WordWrap {
  fn get_words(icu: &IcuEngines, rope_line: &str) -> Vec<(usize, ops::Range<usize>)> {
    let bounds: Vec<usize> = icu.word.segment_str(rope_line).collect();
    let mut cumsum_unicode_width = 0usize;
    bounds
      .windows(2)
      .map(|byte_idx| {
        let byte_idx_range = byte_idx[0]..byte_idx[1];
        cumsum_unicode_width += rope_line[byte_idx_range.clone()].to_string().width();
        (cumsum_unicode_width, byte_idx_range)
      })
      .collect()
  }

  /// In `words: &[(usize, ops::Range<usize>)]` the first item is unicode_width and second is range of the word in byte idx.
  fn get_nearmost_to(words: &[(usize, ops::Range<usize>)], viewport_width: usize) -> FitType {
    // A rope string can be empty so clealry return early.
    if words.is_empty() {
      return FitType::Empty;
    }

    // .partition_point returns the element idx where the match becomes false.
    // For e.g on a word vector of a small single word this returns elememt idx 1 ecen though there is no element idx 1.
    // Note: If no value matches for the given condition `word_width.0 <= viewport_width` then it return element idx 0.
    let mut word_idx = words.partition_point(|word_width| word_width.0 <= viewport_width);

    // An element idx of 1 is ambigious as:
    if word_idx == 1 {
      // 1. There is only 1 word in words vector and it fits inside row.
      if words.len() == 1 {
        return FitType::Whole;
      }

      // 2. There can be many words and only the 1st word fits into row.
      if words.len() > 1 {
        // real_idx = word_idx - 1
        let break_at = words.get(word_idx.saturating_sub(1)).unwrap().1.end;
        return FitType::Slice(word_idx.saturating_sub(1), break_at);
      }
    }

    // Nothing matched -> Overflow
    if word_idx == 0 {
      return FitType::Overflow;
    }

    // else it is a normal case.
    // Do get the exact element idx which gave the last true for the given condition.
    word_idx = word_idx.saturating_sub(1);

    // If element idx is last then the string fits well.
    if word_idx == words.len().saturating_sub(1) {
      return FitType::Whole;
    }
    let break_at = words.get(word_idx).unwrap().1.end;

    FitType::Slice(word_idx, break_at)
  }

  /// Note: You have to put the words vector from outside as this function does loops repeatedly unless the line is fully wrapped.
  fn wrap(
    &mut self,
    icu: &IcuEngines,
    rope_line: &str,
    viewport_width: usize,
    wrapped_rope: &mut WrappedRope,
  ) {
    // Generate internally.
    let words = &Self::get_words(icu, rope_line);
    // The byte idx to break line at.
    let fit_type = Self::get_nearmost_to(words, viewport_width);

    match fit_type {
      FitType::Empty | FitType::Whole => {
        // Note: Since we generate words vector per iteration hence the wrapped_rope is always unique.
        wrapped_rope.wrapped_slices.push(rope_line.into());
        return;
      }

      // Note: This is currently blunt for the overlfowing lines that have more than 1 words.
      // My review: It is fine as I am not going to stare screen for 5 hours to fix it, atleast for now.
      // Fact: The overflowed string can't be empty.
      FitType::Overflow => {
        let overflow_word = Self::get_overflow_word_string(rope_line, words);

        let reaminder = &rope_line[words.get(0).unwrap().1.end..];

        let wrapped_line = Self::wrap_grapheme_level(&overflow_word, viewport_width);

        wrapped_rope.wrapped_slices.extend(wrapped_line);

        if !reaminder.is_empty() {
          self.wrap(icu, reaminder, viewport_width, wrapped_rope);
        }
      }

      FitType::Slice(_word_idx, byte_idx) => {
        let slice = &rope_line[..byte_idx];
        wrapped_rope.wrapped_slices.push(slice.into());
        let remainder = &rope_line[byte_idx..];

        if remainder.width() <= viewport_width {
          wrapped_rope.wrapped_slices.push(remainder.into());
          return;
        }

        // Else break further
        // Note: It automatically stores the value hence no need to worry about unused code.
        self.wrap(icu, remainder, viewport_width, wrapped_rope);
      }
    }
  }
}
