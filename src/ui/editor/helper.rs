use crate::ui::editor::new::core::Editor;

#[allow(dead_code)]
impl Editor {
  pub fn is_last_line(&mut self, target_line_idx: usize) -> bool {
    target_line_idx == self.rope.len_lines() - 1
  }

  pub fn is_multiline(&mut self) -> bool {
    self.rope.len_lines() >= 2
  }

  pub fn get_char_len(&mut self, target_line_idx: usize) -> usize {
    self.rope.line(target_line_idx).len_chars()
  }

  pub fn is_destination_width_small(&mut self, target_line_idx: usize) -> bool {
    let target_char_len = self.get_char_len(target_line_idx);
    if self.is_multiline() {
      if !self.is_last_line(target_line_idx) {
        if self.cursor.0 > target_char_len - 1 {
          return true;
        }
      } else {
        if self.cursor.0 > target_char_len {
          return true;
        }
      }
    }

    false
  }

  pub fn got_to_end(&mut self, target_line_idx: usize) {
    let target_char_len = self.rope.line(target_line_idx).len_chars();

    // In posix last line do not contains line terminating char '\n'
    if self.is_last_line(target_line_idx) {
      self.go_to_line(target_line_idx);
      self.cursor_col_set(target_char_len);
    } else {
      self.go_to_line(target_line_idx);
      self.cursor_col_set(target_char_len.saturating_sub(1));
    }
  }

  pub fn go_to_line(&mut self, target_line_idx: usize) {
    self.cursor.1 = target_line_idx;
  }

  pub fn cursor_col_set(&mut self, char_idx: usize) {
    self.cursor.0 = char_idx;
  }
}
