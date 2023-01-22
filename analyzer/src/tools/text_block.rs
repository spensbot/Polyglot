use owning_ref::RcRef;
use crate::word_db::WordDb;
use iter_tools::Itertools;

pub struct TextBlock {
  text: RcRef<str>,
  total_difficulty: f64,
  max_difficulty: f64,
  word_count: usize,
  whitespace_split_count: usize
}

impl TextBlock {
  pub fn new(text: RcRef<str>, db: &WordDb) -> Self {
    let lower_case = text.to_lowercase();
    let whitespace_split_count = lower_case.split_whitespace().count();
    let difficulties = lower_case.split_whitespace().filter_map(|a| db.difficulty(a)).collect_vec();
    let total_difficulty = difficulties.iter().sum();
    let max_difficulty = difficulties.iter().fold(0.0 as f64, |acc, a| acc.max(*a));
    let word_count = difficulties.len();

    Self {
      text,
      total_difficulty,
      max_difficulty,
      word_count,
      whitespace_split_count
    }
  }

  pub fn new_with_sub_blocks<'a>(text: RcRef<str>, sub_blocks: Vec<&TextBlock>) -> Self {
    let total_difficulty = sub_blocks.iter().map(|a| a.total_difficulty).sum();
    let max_difficulty = sub_blocks.iter().fold(0.0 as f64, |acc, a| acc.max(a.max_difficulty));
    let word_count = sub_blocks.iter().map(|a| a.word_count).sum();
    let whitespace_split_count = sub_blocks.iter().map(|a| a.whitespace_split_count).sum();

    Self {
      text,
      total_difficulty,
      max_difficulty,
      word_count,
      whitespace_split_count
    }
  }

  pub fn max_difficulty(&self) -> f64 {
    self.max_difficulty
  }

  pub fn word_count(&self) -> usize {
    self.word_count
  }

  pub fn average_difficulty(&self) -> f64 {
    self.total_difficulty / self.word_count as f64
  }

  pub fn text(&self) -> &str {
    &self.text
  }

  pub fn valid_word_ratio(&self) -> f64 {
    self.word_count as f64 / self.whitespace_split_count as f64
  }
}

pub fn split_text_ref(text: &RcRef<str>, delimiter: &str, min_bytes: usize) -> impl Iterator<Item = RcRef<str>> {
  let mut split: Vec<RcRef<str>> = Vec::new();

  let mut start_index = 0;
  let mut bytes = 0;

  let indices = text.match_indices(delimiter).collect_vec();
  for (index, str) in indices {
    bytes += str.len();

    if bytes > min_bytes {
      split.push(text.clone().map(|s| &s[start_index..index]));
      start_index = index;
      bytes = 0;
    }
  }
  split.into_iter()
}