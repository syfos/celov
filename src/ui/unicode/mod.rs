use std::ops;

use icu_segmenter::GraphemeClusterSegmenter;
use unicode_bidi::BidiInfo;
use unicode_normalization::{is_nfc, is_nfd};
use unicode_width::UnicodeWidthStr;

use crate::ui::unicode::{bidi_order::BidiAwareLine, grapheme_boundary::GraphemeBoundary};

pub mod bidi_order;
pub mod grapheme_boundary;
pub mod query_normalization;

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

