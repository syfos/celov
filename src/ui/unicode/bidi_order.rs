use unicode_bidi::BidiInfo;

use crate::ui::unicode::unicode_struct::Unicode;
/// Data regarding the bidirectional line for rendering
#[allow(dead_code)]
pub struct BidiAwareLine {
  pub level_number: u8,
  pub is_rtl: bool,
  pub reordered_line: String,
}

#[allow(dead_code)]
impl Unicode {
  /// Flips the LTR character sequences in RTL words into RTL char sequence for correct logical and gramatical display.
  pub fn into_bidirectional_line(rope_line: &str) -> BidiAwareLine {
    // Generate bidirectional information about the rope line
    // Note: Don't try to catch this in struct `IcuEngines` as this one is not an engine and is ment to be recalculated per run.
    let bidi_info = BidiInfo::new(rope_line, None);

    // The internal UBA(Unicode Bidirectional Algorithm) algorith works on Paragraphs.
    //
    // Ropey crate considers paragraph seperators and other line break characters like '\n' equally.
    //
    // E.g of a rope line with paragraph seperator:
    // "Hello, World!<U+2029>"
    //
    // Note: Despite a rope line being a single line with a recognized line termination character, but for the underlying algorithm it is still a single paragraph.
    //
    // Do give a visit to pub enum [`LineBreakChar`] for more information about list of all valid recognized line breaks by ropey.
    let paragraphs = &bidi_info.paragraphs[0];

    // Get the paragraph embedding level number.
    // Odd number means rope line is RTL while,
    // Even number means rope line is LTR
    let level_number = paragraphs.level.number();

    // Check if is the line
    let is_rtl = paragraphs.level.is_rtl();

    // Returns byte indices of paragraph boundary within the given rope line.
    let paragraph_range = paragraphs.range.clone();

    // Reorder the RTL segments correctly.
    // Note:
    // 1. This line is pure cosmetic display
    // 2. The real character sequence will always be LTR beacuse computers don't really store RTL in RTL sequence.
    // 3. The reordered line is purely ment for display purpose.
    //
    // E.g say you want to type : "שלום"
    // But since Hebrew is RTL then
    // You type: ["ם", "ו", "ל", "ש"]
    // Computer stores: ["ם", "ו", "ל", "ש"]
    // Computer displays: ["ש", "ל", "ו", "ם"]
    //
    // So here we are reordering the "unreadable LTR" sequence into "readable RTL" sequence i.e:
    // Computer displays: ["ש", "ל", "ו", "ם"]
    let reordered_line = bidi_info
      .reorder_line(paragraphs, paragraph_range)
      .to_string();

    BidiAwareLine {
      level_number,
      is_rtl,
      reordered_line,
    }
  }
}
