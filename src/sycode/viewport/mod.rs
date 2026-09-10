use std::collections::VecDeque;
/// Stores data about rope lines that are to be displayed on the screen.
#[allow(dead_code)]
pub struct Viewport {
  /// Includes inclusive rope line idx range.
  pub topbot_line_idx: std::ops::RangeInclusive<usize>,
  /// Includes how much rows
  pub row_occupied_by_each_wrapped_line: Vec<std::ops::RangeInclusive<usize>>,
  pub topbot_wrapped_lines: VecDeque<VecDeque<String>>,
  pub height: usize,
  pub width: usize,
}
