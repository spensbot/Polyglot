use std::{collections::HashMap};

const PATH: &str = "./data/hanziDB.txt";

// 0 frequency_rank
// 1 charcter
// 2 pinyin
// 3 definition
// 4 radical
// 5 radical_code
// 6 stroke_count
// 7 hsk_level
// 8 general_standard_num

// A word is a single, contained idea of 1 or more characters
#[derive(Clone, Debug)]
pub struct Character {
  frequency_rank: usize,
  simplified: char,
  pinyin: String,
  definitions: Vec<String>,
  stroke_count: usize,
  hsk_level: usize
}

pub struct HanziDb {
  simplified_map: HashMap<char, Character>,
}

impl HanziDb {
  pub fn load() -> std::result::Result<HanziDb, std::io::Error> {
    let string = std::fs::read_to_string(PATH)?;
    let words: Vec<Character> = string.lines().skip(1).map(|line| line_to_character(line)).flatten().collect();
    let simplified_map = words.into_iter().fold(HashMap::new(), |mut acc, word| {
      acc.insert(word.simplified.clone(), word);
      acc
    });
    
    Ok(HanziDb {
      simplified_map
    })
  }

  pub fn print(&self) {
    self.simplified_map.iter().take(10).for_each(|character| println!("character: {:?}", character))
  }

  pub fn difficulty(&self, char: char) -> f64 {
    self.simplified_map.get(&char).map(|char| char.frequency_rank).unwrap_or_else(|| self.character_count() * 2) as f64
  }

  fn character_count(&self) -> usize {
    self.simplified_map.len()
  }
}

fn usize_from_row_item(row: &Vec<&str>, index: usize) -> Option<usize> {
  let string = row.get(index)?;
  string.parse::<usize>().ok()
}

fn line_to_character(line: &str) -> Option<Character> {
  let row: Vec<&str> = line.split(',').collect();
  let frequency_rank = usize_from_row_item(&row, 0)?;
  let simplified = row.get(1)?.chars().next()?;
  let pinyin = row.get(2)?.to_string();
  let definitions = row.get(3)?.split(';').map(|stref| stref.to_string()).collect();
  let stroke_count = usize_from_row_item(&row, 6)?;
  let hsk_level = usize_from_row_item(&row, 7)?;


  // let split_space: Vec<&str> = line.split(' ').take(2).collect();
  // let traditional = split_space.get(0)?.to_string();
  // let simplified = split_space.get(1)?.to_string();
  // let pinyin = between(line, '[', ']')?.to_string();
  // let definitions = line.split('/').skip(1).map(|stref| stref.to_string()).collect();

  Some(Character { frequency_rank, simplified, pinyin, definitions, stroke_count, hsk_level })
}

