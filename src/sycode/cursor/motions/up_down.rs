use unicode_width::UnicodeWidthStr;

use crate::sycode::{cursor::cursor_struct::Cursor, softwrap::word_wrap::Softwrap};

/// Contains information about the row at which our cursor would land. Helps to smartly prevent the cursor from landing outside the coulumn width of the target row.
#[derive(Default)]
#[allow(dead_code)]
pub struct TargetRowInfo {
  pub line_idx: usize,
  pub string: String,
}

#[allow(dead_code)]
#[allow(clippy::collapsible_if)]
impl Cursor {
  /// Moves the cursor up by the desired rows.
  /// Intentionally clamped at row 0.
  pub fn move_up_in_rows_by(&mut self, softwrap: &Softwrap, target: usize, viewport_height: usize) {
    self.row_pos = self.row_pos.saturating_sub(target);
    let target_data = self.get_target_data(softwrap, target, viewport_height);
    self.adjust_col_pos(target_data);
  }

  /// Moves the cursor down by the desired rows.
  /// Intentionally clamped to the max height of viewport.
  pub fn move_down_in_rows_by(
    &mut self,
    softwrap: &Softwrap,
    target: usize,
    viewport_height: usize,
  ) {
    self.row_pos = self.row_pos.saturating_add(target).min(viewport_height);
    let target_data = self.get_target_data(softwrap, target, viewport_height);
    self.adjust_col_pos(target_data);
  }

  /// Get the target row's rope line index along stribg data where the cursor has to land.
  pub fn get_target_data(
    &mut self,
    softwrap: &Softwrap,
    target: usize,
    viewport_height: usize,
  ) -> TargetRowInfo {
    let mut target_info = TargetRowInfo::default();

    // Return the last slice of the displayed lines for target greater than viewport.
    if target > viewport_height {
      if let Some((idx, value)) = softwrap.display_lines.last_key_value() {
        target_info = TargetRowInfo {
          string: value.slices.last().unwrap().clone(),
          line_idx: *idx,
        }
      }
    }

    // Triggers only if target < viewport_height.
    for (idx, data) in softwrap.display_lines.iter() {
      // Basic checknto skip those ranges that have the end smaller than the target.
      // Let ranges be -> 0..4, 4..9, 9..19
      // Say you've to find -> 6
      // 6 > 4 - 1, skip
      // 6 < 9 - 1, pick
      //
      if target > data.row_range.end.saturating_sub(1) {
        continue;
      }
      // For the end that is equal or less than the target:

      // Given the same example as above we can say:
      // (target - start) + 1 gives the length count under which the slice is standing in the slices vector.
      //
      // (6 - 4) + 1 = 2 + 1 = 3, so => 4, 5, 6
      //
      // So for this key value pair the slices vector at length 3 where the row count becomes 6 is where our string is.
      let range_position = (target - data.row_range.start) + 1;

      // Since, vector is zero indexed then => len - 1 = range_position - 1 = index.
      let slice = data.slices.get(range_position.saturating_sub(1)).unwrap();

      // mutate
      target_info = TargetRowInfo {
        line_idx: *idx,
        string: slice.clone(),
      };
      break;
    }

    target_info
  }

  /// Adjust the col position of cursor if after up/down move it lands on any col > row's col
  fn adjust_col_pos(&mut self, target_info: TargetRowInfo) {
    let string_width = target_info.string.width();
    if self.col_pos.gt(&string_width) {
      self.col_pos = string_width;
    }
  }
}
