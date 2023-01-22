use iter_tools::Itertools;
use owning_ref::RcRef;
use std::rc::Rc;
use crate::{word_db::WordDb, tools::text_block::{TextBlock, split_text_ref}};

const TEXTS_DIR: &str = "../texts/english/books";

pub struct Paragraph {
  text_block: TextBlock,
  sentences: Vec<TextBlock>
}

impl Paragraph {
  pub fn new(text: RcRef<str>, db: &WordDb) -> Self {
    let sentences = split_text_ref(&text, ".", 0)
      .map(|a| TextBlock::new(a, db)).collect_vec();
    let text_block = TextBlock::new_with_sub_blocks(text, sentences.iter().collect());
    
    Self {
      text_block,
      sentences
    }
  }

  pub fn test(&self) {
    println!("\nAll Sentences: {}", self.sentences.len());
    println!("{:<20} {:<20} {:<20} {:<20}", "Word Count", "Max Difficulty", "Avg. Difficulty", "Valid Word Ratio");
    for s in &self.sentences {
      println!("{:<20} {:<20.3} {:<20.3} {:<20.3}", s.word_count(), s.max_difficulty(), s.average_difficulty(), s.valid_word_ratio());
    }
  }
}

pub struct Text {
  name: String,
  text_block: TextBlock,
  paragraphs: Vec<Paragraph>,
}

impl Text {
  pub fn new(name: String, content: String, db: &WordDb) -> Self {
    let text = RcRef::<str>::new(Rc::from(content));
    
    let paragraphs = split_text_ref(&text, "\n", 100)
      .map(|a| Paragraph::new(a, db)).collect_vec();

    let text_block = TextBlock::new_with_sub_blocks(text, paragraphs.iter().map(|p| &p.text_block).collect());

    Self {
      name, 
      text_block,
      paragraphs
    }
  }

  pub fn test(&self) {
    println!("\nAll Paragraphs: {}", self.paragraphs.len());
    println!("{:<20} {:<20} {:<20} {:<20} {:<20}", "Word Count", "Sentence Count", "Max Difficulty", "Avg. Difficulty", "Valid Word Ratio");
    for p in &self.paragraphs {
      println!("{:<20} {:<20} {:<20.3} {:<20.3} {:<20.3}", p.text_block.word_count(), p.sentences.len(), p.text_block.max_difficulty(), p.text_block.average_difficulty(), p.text_block.valid_word_ratio());
    }

    println!("\n13th Paragraph");
    self.paragraphs.get(13).unwrap().test()
  }
}

pub struct Texts {
  vec: Vec<Text>
}

impl Texts {
  pub fn load(db: &WordDb) -> Self {
    let vec = load_texts_as_strings().map(|(name, content)| Text::new(name, content, db)).collect();
    Texts {
      vec
    }
  }

  pub fn test(&self, db: &WordDb) {
    println!("\nAll Texts");
    println!("{:<50} {:<20} {:<20} {:<20} {:<20} {:<20}", "Text", "Word Count", "Paragraphs", "Max Difficulty", "Avg. Difficulty", "Valid Word Ratio");
    for text in &self.vec {
      println!("{:<50} {:<20} {:<20} {:<20.4} {:<20.4} {:<20.2}", text.name, text.text_block.word_count(), text.paragraphs.len(), text.text_block.max_difficulty(), text.text_block.average_difficulty(), text.text_block.valid_word_ratio());
    }

    println!("\n8th Text");
    self.vec.get(8).unwrap().test();

    let sentences = self.vec.iter().map(|t| t.paragraphs.iter().map(|p| p.sentences.iter().filter(|s| s.word_count() > 10 && s.word_count() < 100 && s.max_difficulty() < 0.01 && s.valid_word_ratio() > 0.99).collect_vec())).flatten().flatten().sorted_by_key(|a| (a.average_difficulty() * 10000000.0) as usize).collect_vec();
    
    println!("\nEasiest 5 sentences");
    for s in sentences.iter().take(5) {
      print_sentence(s, db)
    }

    println!("\nHardest 5 sentences");
    for s in sentences.iter().rev().take(5) {
      print_sentence(s, db)
    }
  }
}

pub fn load_texts_as_strings() -> impl Iterator<Item = (String, String)> {
  std::fs::read_dir(TEXTS_DIR).unwrap().map(|entry| {
    let path = entry.unwrap().path();
    let name = path.file_name().map(|name| name.to_string_lossy().to_string()).unwrap_or_else(|| "Bad Filename".to_string());

    let text = std::fs::read_to_string(path).unwrap();

    (name, text)
  })
}

fn print_sentence(s: &TextBlock, db: &WordDb) {
  println!("\n {}", s.average_difficulty());
  println!("{}", db.print_with_difficulty(s.text()));
}