use icu_segmenter::{LineSegmenter, options::LineBreakOptions};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

fn main() {
  let rope_line = "Hello, World! I am a good Guy. I a a very Good Person. Lsps.\n";

  get_breakpoints(rope_line);

  println!("Wrap lines: ");
  for i in wrap(rope_line, &40usize) {
    println!("{i:?}");
  }

  get_row_ranges();

  let row_taken = wrap(rope_line, &40usize).len();
  println!("{row_taken}");
}

/// Returns ranges of wrapped lines, telling exactly how many rows of viewport have been occupied by a particular line.
/// Note: each element of `wrapped_lines` and the returned vector is equal to a rope line.
fn get_row_ranges() -> Vec<std::ops::RangeInclusive<usize>> {
  let wrapped_lines_len: Vec<usize> = vec![3, 5, 2];
  let mut start = 1usize;
  let mut row_ranges = Vec::new();
  for wrapped_line in wrapped_lines_len {
    let end = start + wrapped_line.saturating_sub(1);
    row_ranges.push(start..=end);
    start = end + 1;
  }
  println!("{row_ranges:?}");
  row_ranges
}

/// Returns the vector containing breakpoints of given string.
/// Note: the breakpoints are `unicode-aware`, `grapheme-aware` and more specifically `scripto continua-aware`
fn get_breakpoints(rope_line: &str) -> Vec<usize> {
  LineSegmenter::new_auto(LineBreakOptions::default())
    .segment_str(rope_line)
    .collect()
}

/// Get the slices at valid break points of strings.
#[allow(dead_code)]
fn get_breakpoint_slices(rope_line: &str, breakpoints: &[usize]) -> Vec<String> {
  breakpoints
    .windows(2)
    .map(|w| rope_line[w[0]..w[1]].to_string())
    .collect()
}

/// Returns the slices of a line for softwrap.
/// Note: Only the last value will contain a linebreak char/unicode.
#[allow(dead_code)]
fn wrap(rope_line: &str, viewport_width: &usize) -> Vec<String> {
  let breakpoints = &get_breakpoints(rope_line);
  let mut wrap = Vec::new();
  let breakpoint_slices = get_breakpoint_slices(rope_line, breakpoints);

  let mut current_line = String::new();
  let mut current_width = 0usize;

  for str in breakpoint_slices.iter() {
    let slice_width = str.width_cjk();

    if slice_width > *viewport_width {
      if !current_line.is_empty() {
        wrap.push(std::mem::take(&mut current_line));
        current_width = 0;
      }
      wrap.extend(break_at_grapheme(str, viewport_width));
    } else if current_width + slice_width > *viewport_width {
      wrap.push(std::mem::take(&mut current_line));
      current_line.push_str(str);
      current_width = slice_width;
    } else {
      current_line.push_str(str);
      current_width += slice_width;
    }
  }

  if !current_line.is_empty() {
    wrap.push(current_line);
  }

  wrap
}

fn break_at_grapheme(slice: &str, viewport_width: &usize) -> Vec<String> {
  //
  let grapheme_aware_break = most_equal(&get_cumulative_widths_of_graphemes(slice), viewport_width);

  let mut wrap = Vec::new();
  wrap.push(slice[..grapheme_aware_break].to_string());

  let remainder = &slice[grapheme_aware_break..];
  // loops for all the cases where the remainer
  // would be wider than viewport_width
  // Note: The last value will be always dropped
  if remainder.width_cjk() > *viewport_width {
    wrap.extend(break_at_grapheme(remainder, viewport_width));
  }
  // This will catch such remainder whose terminal
  // width is lesser than viewport width
  else {
    wrap.push(remainder.to_string());
  }

  wrap
}

/// Holds `cumulative`width of a grapheme of a slice string
/// along the byte index the grapheme belongs to.
pub struct SliceData {
  pub byte_idx: usize,
  pub grapheme_cumulative_width: usize,
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
