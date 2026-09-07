use std::ops;

use icu_segmenter::GraphemeClusterSegmenter;
use unicode_bidi::BidiInfo;
use unicode_normalization::{is_nfc, is_nfd};
use unicode_width::UnicodeWidthStr;

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
#[derive(Default, Debug, Clone)]
pub struct GraphemeBoundary {
  pub line_idx: usize,

  /// Tells width of current grapheme boundary in terms of `viewport's cells`.
  /// Helps in movement of cursor.
  /// ```
  /// // Usage:
  /// let new_col_pos = cursor_col + next_grapheme.width;
  /// let new_col_pos = cursor_col - prev_grapheme.width;
  /// ```
  pub width: usize,

  /// The cumulative width sum for the current grapheme boundary of the rope string.
  /// Why exists?: To help [`Softwrap`] do the wrap math.
  pub cumulative_width: usize,

  /// The rope byte idx range the graphemes stands on.
  /// E.g, say for a CJK string: [0..3, 3..6, 6..9, 9..12]
  /// Your byte boundaries -> [0..=2, 3..=5, 6..=8, 9..=11]
  pub absolute_byte_idx: ops::Range<usize>,
}

impl Unicode {
  /// This function returns [`GraphemeBoundary`] containing useful data related to the rope line.
  ///
  /// Quick overview:
  /// 1. Line index
  /// 2.  Boundary width in viewport cells, for cursor movement.
  /// 3. Cumulative width of boundary, for [`Softwrap`] based softwrap.
  /// 4. Absolute byte index range of the grapheme in the given rope line.
  ///
  /// Why exists?:
  /// Exists because you can't reliably edit/wrap a rope string without knowing the boundaries.
  pub fn get_grapheme_boundary(
    rope_line: &str,
    line_idx: usize,
    line_to_byte: usize,
  ) -> Vec<GraphemeBoundary> {
    // Todo: Move the segementer to a struct to avoid regeneration.
    let segment = GraphemeClusterSegmenter::new();
    // Generate vector containing byte indicies.
    let breakpoints: Vec<usize> = segment.segment_str(rope_line).collect();

    let mut cumulative_width = 0usize;

    // Returns exclusive absolute byte index vector.
    // For e.g: [0..3, 3..4, 4..6]
    //
    // Note: Since this default range excludes the end value, you just use it as:
    // 0..=2, 3..=3, 4..=5 with respective counts as 3, 1, and 2
    breakpoints
      // Yeilds the slices/windows each of length 2.
      // E.g: [0, 3, 6, 9, 12]
      // 0..3, 3..6, 6..9, 9..12
      .windows(2)
      // byte_window is a slice/window of length 2
      // line_to_byte + byte_idx -> generates absolute byte index for rope level operation.
      .map(|byte_window| {
        // Get the relative start and end byte index
        // Why?: To get the grapheme slice.
        let (rel_start, rel_end) = (byte_window[0], byte_window[1]);

        // Todo: Do check is range exclusive even the right choice.
        //
        // Gives the grapheme slice from the rope string.
        let grapheme_slice = &rope_line[rel_start..rel_end];

        // Get width of current grapheme.
        let width = grapheme_slice.width();

        // Get the cumulative width
        // Why?: For grapheme aware soft wrapping.
        cumulative_width += width;

        // Get the absolute byte index.
        // Why?: To work with rope's insert/remove api.
        let absolute_byte_idx = (line_to_byte + byte_window[0])..(line_to_byte + byte_window[1]);

        // Return
        GraphemeBoundary {
          line_idx,
          width,
          cumulative_width,
          absolute_byte_idx,
        }
      })
      // collect and cast
      .collect()
  }
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
