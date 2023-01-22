use crate::db::Word;

// Short texts currently enabled in case the algo is slow (cause it probably will be)
const TEXTS_DIR: &str = "short_texts";
// const TEXTS_DIR: &str = "texts";

const BLOCK_DELIMITERS: [char; 4] = ['，', '：', ',', ':'];
const SENTENCE_DELIMITERS: [char; 2] = ['。', '.'];
const PARAGRAPH_DELIMITERS: [char; 1] = ['\n'];

struct ParsedBlock {
  words: Vec<Word>,
  difficulty: f64
}

impl ParsedBlock {
  pub fn new(words: Vec<Word>) -> Self {
    Self {
      words,
      // TODO
      difficulty: 0.0
    }
  }
}



// Text between punctuation
struct Block {
  string: String,
  parsed_blocks: Vec<ParsedBlock>,
  difficulty: f64
}

impl Block {
  fn new(string: &str) -> Self {
    Self {
      string: string.to_string(),
      // TODO
      parsed_blocks: Vec::new(),
      difficulty: 0.0
    }
  }
}

struct Sentence {
  blocks: Vec<Block>,
  difficulty: f64
}

impl Sentence {
  fn new(string: &str) -> Self {
    let blocks: Vec<Block> = string.split(BLOCK_DELIMITERS).filter(|s| s.len() > 0).map(|s| Block::new(s)).collect();
    let difficulty = blocks.iter().fold(0.0, |acc, b| acc + b.difficulty);
    Self {
      blocks,
      difficulty
    }
  }
}

struct Paragraph {
  sentences: Vec<Sentence>,
  difficulty: f64
}

impl Paragraph {
  fn new(string: &str) -> Self {
    let sentences: Vec<Sentence> = string.split(SENTENCE_DELIMITERS).filter(|s| s.len() > 0).map(|s| Sentence::new(s)).collect();
    let difficulty = sentences.iter().fold(0.0, |acc, s| acc + s.difficulty);
    Self {
      sentences,
      difficulty
    }
  }
}

struct Text {
  string: String,
  paragraphs: Vec<Paragraph>,
  difficulty: f64
}

impl Text {
  fn new(string: &str) -> Self {
    let paragraphs: Vec<Paragraph> = string.split(PARAGRAPH_DELIMITERS).filter(|s| s.len() > 0).map(|s| Paragraph::new(s)).collect();
    let difficulty = paragraphs.iter().fold(0.0, |acc, p| acc + p.difficulty);

    Self {
      string: string.to_string(),
      paragraphs,
      difficulty
    }
  }
}

pub struct Texts {
  texts: Vec<Text>
}

impl Texts {
  pub fn load() -> std::result::Result<Self, std::io::Error> {
    let texts = texts_as_strings().into_iter().map(|text_string| Text::new(&text_string)).collect();

    Ok(Texts {
      texts
    })
  }
}

pub fn texts_as_strings() -> Vec<String> {
  std::fs::read_dir(TEXTS_DIR).unwrap().map(|entry| {
    let path = entry.unwrap().path();

    let raw_string = std::fs::read_to_string(path).unwrap();

    raw_string.chars().collect()
  }).collect()
}