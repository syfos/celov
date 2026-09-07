use icu_segmenter::{
  GraphemeClusterSegmenter, GraphemeClusterSegmenterBorrowed, WordSegmenter, WordSegmenterBorrowed,
  options::WordBreakInvariantOptions,
};

/// Contains the heavier Icu4x engines that are expensive to compile more than once.
/// Note: Caching any of the listed doesn't causes staleness because these are engines not some random methods.
pub struct IcuEngines {
  pub word: WordSegmenterBorrowed<'static>,
  pub grapheme_cluster: GraphemeClusterSegmenterBorrowed<'static>,
}

#[allow(dead_code)]
impl IcuEngines {
  /// Generate icu4x expensive engines.
  pub fn new() -> Self {
    Self {
      word: WordSegmenter::new_auto(WordBreakInvariantOptions::default()),
      grapheme_cluster: GraphemeClusterSegmenter::new(),
    }
  }
}
