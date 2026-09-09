use crate::ui::editor::new::core::Editor;

#[allow(dead_code)]
impl Editor {
  pub fn insert_char(&mut self, char: char) {
    let cursor_idx = self.rope.line_to_char(self.cursor.1) + self.cursor.0;
    self.rope.insert_char(cursor_idx, char);
    self.increment_cursor_col(1);
  }

  pub fn new_line(&mut self) {
    // absolute index of cursor in whole document
    let absolute_cursor_idx = self.rope.line_to_char(self.cursor.1) + self.cursor.0;

    // this inserts line terminating char at cursor_idx
    self.rope.insert_char(absolute_cursor_idx, '\n');

    // move down and set cursor col to 0
    self.cursor.1 += 1;
    self.cursor.0 = 0;
  }

  pub fn remove_char(&mut self) {
    if self.cursor.1 == 0 && self.cursor.0 == 0 {
      return;
    }

    if self.cursor.1 >= 1 && self.cursor.0 == 0 {
      let prev_line = self.rope.line(self.cursor.1.saturating_sub(1));
      let char_len_of_prev_line_before_merge = prev_line.len_chars();

      let char_idx_of_previous_posix_line = self.rope.line_to_char(self.cursor.1.saturating_sub(1))
        + char_len_of_prev_line_before_merge;

      self
        .rope
        .remove(char_idx_of_previous_posix_line - 1..char_idx_of_previous_posix_line);
      self.decrement_cursor_row(1);
      self.cursor.0 = char_len_of_prev_line_before_merge - 1;
      return;
    }

    let cursor_idx = self.rope.line_to_char(self.cursor.1) + self.cursor.0;

    self.rope.remove(cursor_idx.saturating_sub(1)..cursor_idx);
    self.decrement_cursor_col(1);
  }
}
