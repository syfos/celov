use icu_segmenter::{WordSegmenter, options::WordBreakInvariantOptions};

fn main() {
  let s1 = "Hello, World";
  // [0, 5, 6, 7, 12]
  let s2 = "你好世界";
  // [0, 6, 12]
  let s3 = "こんにちは世界";
  // [0, 15, 21]
  let s4 = "สวัสดีชาวโลก";
  // [0, 18, 27, 36]
  let s5 = "សួស្តី\u{200B}ពិភពលោក";
  // [0, 18, 21, 42]
  let s6 = "ສະບາຍດີໂລກ";
  // [0, 15, 21, 30]
  let s7 = "မင်္ဂလာပါကမ္ဘာလောက";
  // [0, 21, 27, 42, 54]

  let s = s5;
  let segmenter = WordSegmenter::new_auto(WordBreakInvariantOptions::default());
  let bounds: Vec<usize> = segmenter.segment_str(s).collect();
  println!("{:?}", bounds);
}
