use std::collections::HashMap;
use iter_tools::Itertools;
use crate::tools::dict::Dict;

#[derive(Clone, Debug)]
pub struct WordInfo {
  definition: String,
  occurances: u64,
  rank_ratio: f64,
}

impl WordInfo {
  pub fn difficulty(&self) -> f64 {
    self.rank_ratio.powf(1.0)
  }
}

pub struct WordDb {
  map: HashMap<String, WordInfo>
}

impl WordDb {
  pub fn load() -> Self {
    let mut map: HashMap<String, WordInfo> = Dict::load().into_definitions().into_iter().map(|(word, definition)| {
      (word, WordInfo {
        definition,
        occurances: 0,
        rank_ratio: 0.0
      })
    }).collect();

    for (_, text) in crate::texts::load_texts_as_strings() {
      // Conversion to Utf32 is necessary, otherwise you end up iterating over bytes
      for word in text.split_whitespace() {
        if let Some(info) = map.get_mut(word) {
          info.occurances += 1
        }
      }
    }

    let max_rank = map.len();
    for (index, (_word, info)) in map.iter_mut().sorted_by_key(|(_word, info)| info.occurances).rev().enumerate() {
      info.rank_ratio = index as f64 / max_rank as f64
    }

    Self {
      map
    }
  }

  pub fn define(&self, word: &str) -> Option<&str> {
    self.map.get(word).map(|info| &info.definition as &str)
  }

  pub fn difficulty(&self, word: &str) -> Option<f64> {
    self.map.get(word).map(|info| info.difficulty())
  }

  pub fn test(&self) {
    println!("\nDB Test:");
    for (word, info) in self.map.iter().sorted_by_key(|(_word, info)| info.occurances).rev().take(1000) {
      println!("  {:<13} --> {:<7} | {:0.5}", word, info.occurances, info.difficulty());
    }
  }

  pub fn map(&self) -> &HashMap<String, WordInfo> {
    &self.map
  }

  pub fn print_with_difficulty(&self, s: &str) -> String {
    let lower = s.to_lowercase();

    lower.split_whitespace().map(|word| {
      if let Some(dif) = self.difficulty(word) {
        format!("{} ({:0.4}) ", word, dif)
      } else {
        format!("{} ", word)
      }
    }).collect()
  }
}