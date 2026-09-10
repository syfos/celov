use ratatui::{
  Frame,
  layout::Rect,
  text::{Line, Text},
  widgets::Paragraph,
};

use crate::sycode::core::{Cursor, Editor};
impl Editor {
  pub fn render_rope(&mut self, frame: &mut Frame, area: Rect) {
    let net_lines = self.rope.len_lines();
    let start_line = self.scroll_offset;
    let end_line = (start_line + area.height as usize).min(net_lines);

    let lines: Vec<ratatui::text::Line> = (start_line..end_line)
      .map(|i| {
        let rope_slice = self.rope.line(i).to_string();
        let escaped = Self::escape_hidden_chars(&rope_slice);
        Line::raw(escaped)
      })
      .collect();

    let paragraph = Paragraph::new(Text::from(lines));
    frame.render_widget(paragraph, area);

    let screen_row = area.y + (self.cursor.1 - start_line) as u16;
    let screen_col = area.x + self.cursor.0 as u16;

    // frame.set_cursor_position(Position::new(screen_col, screen_row));
    Cursor::render(&mut self.cursor, &self.mode, frame);
  }

  fn escape_hidden_chars(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
      match c {
        '\n' => out.push_str("\\n"),
        '\r' => out.push_str("\\r"),
        '\t' => out.push_str("\\t"),
        '\0' => out.push_str("\\0"),
        _ => out.push(c),
      }
    }
    out
  }
}
