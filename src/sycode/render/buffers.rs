use crate::sycode::{core::Editor, unicode::unicode_struct::Unicode};
use ratatui::{
  Frame,
  text::{Line, Text},
  widgets::Paragraph,
};

impl Editor {
  pub fn display_rope(&mut self, frame: &mut Frame) -> Result<(), Box<dyn std::error::Error>> {
    self.softwrap.wrap_viewport_lines(
      &mut self.rope,
      0usize..=1usize,
      &self.icu,
      self.viewport_width,
    );

    let mut lines: Vec<ratatui::text::Line> = Vec::new();
    for (_k, v) in self.softwrap.display_lines.iter() {
      for slice in v.slices.iter() {
        let escaped_trailing_char = Self::escape_hidden_chars(slice);
        let bidi_line = Unicode::into_bidirectional_line(&escaped_trailing_char).reordered_line;
        lines.push(Line::raw(bidi_line));
      }
    }
    let paragraph = Paragraph::new(Text::from(lines));
    frame.render_widget(paragraph, frame.area());
    self.cursor.render(&self.mode, frame, false)?;
    Ok(())
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
