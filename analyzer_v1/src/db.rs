use std::collections::HashMap;
use iter_tools::Itertools;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;
use crate::hanzi_db::HanziDb;
use crate::texts::texts_as_strings;
use widestring::Utf32String;

const PATH: &str = "./data/cedict_1_0_ts_utf-8_mdbg.txt";
const MAX_SEARCH_LEN: usize = 7;

// A word is a single, contained idea of 1 or more characters
#[derive(Clone, Debug)]
pub struct Word {
  traditional: String,
  simplified: String,
  pinyin: String,
  definitions: Vec<String>,
  occurances: usize,
  occurance_rank: usize,
  difficulty: f64 // higher = more difficult
}

impl Word {
  pub fn update_difficulty(&mut self, hanzi_db: &HanziDb) -> f64 {
    self.difficulty = self.occurance_rank as f64 + self.simplified.chars().fold(0.0, |acc, char| {
      acc + hanzi_db.difficulty(char) as f64
    });
    self.difficulty
  }

  pub fn to_string(&self) -> String {
    format!("{} {}x ({}) {}: {}", self.simplified, self.occurances, self.traditional, self.pinyin, self.definitions.first().unwrap())
  }

  pub fn print(&self) {
    println!("{}", self.to_string())
  }
}

#[derive()]
pub struct Db {
  // traditional_map: HashMap<String, Word>,
  simplified_map: HashMap<String, Word>,
  max_word_len: usize,
}

impl Db {
  pub fn load() -> std::result::Result<Self, std::io::Error> {
    let string = std::fs::read_to_string(PATH)?;
    let words: Vec<Word> = string.lines().filter(|line| is_valid_line(line)).map(|line| line_to_word(line)).flatten().collect();
    let max_word_len = words.iter().map(|w| w.simplified.len()).max().unwrap_or(0);
    
    let mut simplified_map = words.into_iter().fold(HashMap::new(), |mut acc, word| {
      acc.insert(word.simplified.clone(), word);
      acc
    });

    let hanzi_db = HanziDb::load()?;

    for text in crate::texts::texts_as_strings() {
      // Conversion to Utf32 is necessary, otherwise you end up iterating over bytes of utf-8
      let text_utf32 = Utf32String::from(text);
      println!("Text len: {}", text_utf32.len());
      for start_index in 0..text_utf32.len() {
        for len in 1..MAX_SEARCH_LEN {
          let stop_index = (start_index + len).min(text_utf32.len()-1);
          let search_slice = &text_utf32[start_index..stop_index];
          if let Some(word) = simplified_map.get_mut(&search_slice.to_string()) {
            word.occurances += 1
          }
        }
      }
    }

    // Set occurance ranks
    let mut vec: Vec<&mut Word> = simplified_map.iter_mut().map(|(_, word)| word).collect();
    vec.sort_by_key(|a| a.occurances);
    for (rank, word) in vec.iter_mut().enumerate() {
      word.occurance_rank = rank
    }


    for (_, word) in &mut simplified_map {
      word.update_difficulty(&hanzi_db);
    }

    Ok(Self {
      simplified_map,
      max_word_len
    })
  }

  pub fn print(&self) {
    println!("word count: {}", self.word_count());
    println!("max word len: {}", self.max_word_len);

    println!("max occurences: {}", self.simplified_map.iter().map(|(_, word)| word.occurances).max().unwrap());

    let mut vec: Vec<&Word> = self.simplified_map.iter().map(|(_, word)| word).collect();
    vec.sort_by(|a, b| a.difficulty.total_cmp(&b.difficulty));

    // vec.iter().take(5).for_each(|word| println!("word: {:?}", word));

    self.simplified_map.iter().sorted_by_key(|(_, w)| -(w.occurances as i64)).take(5).for_each(|(_, w)| w.print())
  }

  pub fn word_count(&self) -> usize {
    self.simplified_map.len()
  }
}

fn is_valid_line(ln: &str) -> bool {
  return !ln.starts_with("#")
}

fn between(string: &str, start: char, end: char) -> Option<&str> {
  let i_start = string.find(start)?;
  let i_end = string.find(end)?;
  return Some(&string[i_start..i_end])
}
 
fn line_to_word(line: &str) -> Option<Word> {
  let split_space: Vec<&str> = line.split(' ').take(2).collect();
  let traditional = split_space.get(0)?.to_string();
  let simplified = split_space.get(1)?.to_string();
  let pinyin = between(line, '[', ']')?.to_string();
  let definitions = line.split('/').skip(1).map(|stref| stref.to_string()).collect();

  Some(Word {
    traditional,
    simplified,
    pinyin,
    definitions,
    // These must be set later
    occurances: 0,
    occurance_rank: 0,
    difficulty: 0.0
  })
}

