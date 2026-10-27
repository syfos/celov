use crate::sycode::{core::Editor, unicode::unicode_struct::Unicode};
use ratatui::{
  Frame,
  text::{Line, Text},
  widgets::Paragraph,
};

impl Editor {
  pub fn display_rope(&mut self, frame: &mut Frame) -> Result<(), Box<dyn std::error::Error>> {
    self.softwrap.wrap_lines_for_viewport(
      &mut self.rope,
      &mut 0,
      &self.icu,
      self.viewport_width,
      self.viewport_height,
    );

    let mut lines: Vec<ratatui::text::Line> = Vec::new();
    for (_k, v) in self.softwrap.display_lines.iter() {
      for slice in v.slices.iter() {
        let debugger_slice = Self::escape_hidden_chars(slice);

        // into_bidrectional_line() can have an internal panic for empty string.
        if !debugger_slice.is_empty() {
          let bidi_line = Unicode::into_bidirectional_line(&debugger_slice).reordered_line;
          lines.push(Line::raw(bidi_line));
        } else {
          lines.push(Line::raw(debugger_slice));
        }
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
