use celov::sycode::{core::Editor, softwrap::word_wrap::WrappedRope};
use unicode_width::UnicodeWidthStr;

fn main()-> Result<(), Box<dyn std::error::Error>>{
  let mut editor = Editor::new()?;
  let eng_par_simple = "Every morning, the quiet village wakes beneath golden sunlight. Birds sing from nearby trees, children walk to school, and farmers begin their work with cheerful hearts.";
  
  let eng_par_complex = "Antidisestablishmentarianism-inspired counterrevolutionaries incomprehensibility-related telecommunications-based institutionalization-focused internationalization-oriented commercialization-driven constitutionalization-centered interdisciplinarity-dependent psychopharmacological neurodevelopmentally-informed electroencephalographically-monitored gastrointestinally-associated immunohistochemistry-supported environmentalism-consciousness representationalistically-oriented photosynthetically-generated metalinguistically-informed circumstantiality-sensitive electromagneticism-related mechanistically-grounded technologically-advanced organizationally-coordinated transformationally-focused computationally-assisted experimentally-validated statistically-supported mathematically-demonstrated interdisciplinary-research.";

  let eng_overflow = "thequickbrownfoxjumpedoverthelazydogrepateadlyanddisturbeditssleepmakingthedogangry";

  let con_1 = "कर्मण्येवाधिकारस्तेमाफलेषुकदाचनकर्मफलहेतुर्भूर्मातेसङ्गोऽस्त्वकर्मणि";
  let con_2 = "吾輩は猫である名前はまだ無いどこで生れたかとんと見当がつかぬ何でも薄暗いじめじめした所でニャーニャー泣いていた事だけは記憶している";

  let s = con_2;

  let breakpoints: Vec<usize> = editor.icu.word.segment_str(s).collect();

  let mut wrappings = WrappedRope::default();
  editor.softwrap.wrap(&editor.icu, s, 40, &mut wrappings);
  for slice in wrappings.slices.iter() {
    println!("{}",slice.width());
  }
  println!("{:#?}", wrappings.slices);
  
  Ok(())

}
