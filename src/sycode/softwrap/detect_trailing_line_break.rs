use crate::sycode::core::Editor;
/// Contains list of all recognized line break charachters that [`Ropey`] uses to mark a string as a [`RopeSlice`.]
/// Note: This is just to map the rendered viewport line to let the caller easiky know what is the only line break character of this line.
#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
#[allow(nonstandard_style)]
pub enum LineBreakChar {
  LineFeed,
  FromFeed,
  CarriageReturn,
  CarriageReturn_LineFeed,
  VerticalTab,
  ParagraphSeperator,
  LineSeperator,
  NextLine,
  None,
}

impl Editor {
  /// Matches the last most line break unicode character and returns one of variant of [`LineBreakChar`].
  fn detect_trailing_linebreak_char(rope_line: &str) -> LineBreakChar {
    if rope_line.ends_with("\r\n") {
      return LineBreakChar::CarriageReturn_LineFeed;
    }

    match rope_line.chars().last() {
      Some('\n') => LineBreakChar::LineFeed,
      Some('\r') => LineBreakChar::CarriageReturn,
      Some('\u{0B}') => LineBreakChar::VerticalTab,
      Some('\u{0C}') => LineBreakChar::FromFeed,
      Some('\u{85}') => LineBreakChar::NextLine,
      Some('\u{2028}') => LineBreakChar::LineSeperator,
      Some('\u{2029}') => LineBreakChar::ParagraphSeperator,
      _ => LineBreakChar::None,
    }
  }
}
