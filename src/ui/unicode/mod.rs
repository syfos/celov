use std::ops;

use icu_segmenter::GraphemeClusterSegmenter;
use unicode_bidi::BidiInfo;
use unicode_normalization::{is_nfc, is_nfd};
use unicode_width::UnicodeWidthStr;

use crate::ui::unicode::grapheme_boundary::GraphemeBoundary;

pub mod grapheme_boundary;

pub enum CanonicalType {
  /// String contains `NFC` along `NFD`.
  Mix,
  /// String contains only `NFC`
  Nfc,
  /// String contains only `NFD`
  Nfd,
  /// String conatins neither of `NFC` or `NFD`
  None,
}

/// Gives Unicode support to Sycode.
#[allow(dead_code)]
pub struct Unicode {
  /// Viewport lines into Grapheme aware lines
  pub viewport_grapheme_lines: Vec<Vec<GraphemeBoundary>>,
  /// Viewport lines into Bidirection aware lines
  pub viewport_bidirectional_lines: Vec<BidiAwareLine>,
}

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

  /// This function returns the `[CanonicalType]` of Normalization form of the given string.
  ///
  /// This function is purely for search/replace command.
  /// ```
  /// use unicode_normalization::{is_nfc, is_nfd};
  ///
  /// match (is_nfc(query), is_nfd(query)) {
  ///   // Means there is no NFC and NFD
  ///   (true, true) => CanonicalType::None,
  ///   // Means there is only NFC
  ///   (true, false) => CanonicalType::Nfc,
  ///   // Means there is only NFD
  ///   (false, true) => CanonicalType::Nfd,
  ///   // Means there is both
  ///   (false, false) => CanonicalType::Mix,
  /// }
  /// ```
  ///
  pub fn check_canonical_form(query: &str) -> CanonicalType {
    match (is_nfc(query), is_nfd(query)) {
      // Means there is no NFC and NFD
      (true, true) => CanonicalType::None,
      // Means there is only NFC
      (true, false) => CanonicalType::Nfc,
      // Means there is only NFD
      (false, true) => CanonicalType::Nfd,
      // Means there is both
      (false, false) => CanonicalType::Mix,
    }
  }
}
