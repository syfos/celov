use std::ops;

use icu_segmenter::GraphemeClusterSegmenter;
use unicode_bidi::BidiInfo;
use unicode_normalization::{is_nfc, is_nfd};
use unicode_width::UnicodeWidthStr;

use crate::ui::unicode::{bidi_order::BidiAwareLine, grapheme_boundary::GraphemeBoundary};

pub mod bidi_order;
pub mod grapheme_boundary;
pub mod query_normalization;
pub mod unicode_struct;


