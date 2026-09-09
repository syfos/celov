use crate::ui::editor::new::core::Editor;

impl Editor {
  /// increment cursor col by the given number, clamp at length of cursor row.
  pub fn increment_cursor_col(&mut self, col: usize) {
    let current_line_char_len = self.rope.line(self.cursor.1).len_chars();
    if self.rope.len_lines() != 1
      && self.rope.len_lines() >= 2
      && self.cursor.1 < self.rope.len_lines() - 1
    {
      self.cursor.0 = self
        .cursor
        .0
        .saturating_add(col)
        .min(current_line_char_len.saturating_sub(1));
      return;
    }
    self.cursor.0 = self.cursor.0.saturating_add(col).min(current_line_char_len);
  }

  /// increment cursor row by the given number, clamp at last line.
  pub fn increment_cursor_row(&mut self, row: usize) {
    if self.rope.len_lines() >= 2
      && self.cursor.0 == self.rope.line(self.cursor.1).len_chars().saturating_sub(1)
    {
      if self.cursor.1 == self.rope.len_lines() - 2 {
        // chars of next line
        let last_line_char_len = self.rope.line(self.cursor.1 + 1).len_chars();

        // set cursor row
        self.cursor.1 = self
          .cursor
          .1
          .saturating_add(row)
          .min(self.rope.len_lines() - 1);

        // set cursor column
        self.cursor.0 = last_line_char_len;
        return;
      }
      // chars of next line
      let next_line_char_len = self.rope.line(self.cursor.1 + 1).len_chars();

      // set cursor row
      self.cursor.1 = self
        .cursor
        .1
        .saturating_add(row)
        .min(self.rope.len_lines() - 1);

      // set cursor column
      self.cursor.0 = next_line_char_len.saturating_sub(1);
      return;
    }

    self.cursor.1 = self
      .cursor
      .1
      .saturating_add(row)
      .min(self.rope.len_lines().saturating_sub(1));
  }

  /// Decrement cursor column by given number, clamps at 0.
  pub fn decrement_cursor_col(&mut self, col: usize) {
    self.cursor.0 = self.cursor.0.saturating_sub(col);
  }

  /// Decrement cursor row by given number, clamps at 0.
  pub fn decrement_cursor_row(&mut self, row: usize) {
    if self.rope.len_lines() >= 2 {
      // if cursor is at end of line for any line except last.
      if self.cursor.0 == self.rope.line(self.cursor.1).len_chars() - 1
        && self.cursor.1 != self.rope.len_lines() - 1
      {
        self.cursor.1 = self.cursor.1.saturating_sub(row);
        self.cursor.0 = self.rope.line(self.cursor.1).len_chars() - 1;
        return;
      }

      // if cursor line is at very last line and at very last char
      if self.cursor.1 == self.rope.len_lines() - 1
        && self.cursor.0 == self.rope.line(self.cursor.1).len_chars()
      {
        self.cursor.1 = self.cursor.1.saturating_sub(row);
        self.cursor.0 = self.rope.line(self.cursor.1).len_chars() - 1;
        return;
      }
    }
    self.cursor.1 = self.cursor.1.saturating_sub(row);
  }
}
