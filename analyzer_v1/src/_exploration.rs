use unicode_segmentation::UnicodeSegmentation;
use widestring::{Utf16String, Utf32String};

struct Tests {
  ascii: &'static str,
  hanzi: &'static str,
  emoji: &'static str
}

fn tests() -> Tests {
  Tests {
    ascii: "abcd",
    hanzi: "僧行六人",
    emoji: "😍✨🎉🔥"
  }
}

pub fn explore() {
  for_each("print",|s| s.into());
  for_each("len()", |s| s.len().to_string());
  for_each("chars", |s| format!("{:?}", s.chars()));
  for_each("graphemes", |s| {
    let gs: Vec<usize> = s.graphemes(true).map(|g| g.len()).collect();
    format!("{:?}", gs)
  });

  for_each("Utf16String len", |s| format!("{}", Utf16String::from(s).len()));

  for_each("Utf32String len", |s| format!("{}", Utf32String::from(s).len()));
}

fn for_each(name: &str, f: fn(&str) -> String) {
  println!("\n{}", name);
  let tests = tests();
  println!("ascii: {}", f(tests.ascii));
  println!("hanzi: {}", f(tests.hanzi));
  println!("emoji: {}", f(tests.emoji));
}