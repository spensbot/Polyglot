use std::collections::HashMap;
use crate::word_db::WordDb;
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Learning {
  seen: usize,
  searched: usize,
  acc: i64
}

impl Default for Learning {
  fn default() -> Self {
      Learning { seen: 0, searched: 0, acc: 0 }
  }
}

pub enum LearningStatus {
  NoSearch,
  Search
}

#[derive(Serialize, Deserialize)]
pub struct UserLearning {
  words: HashMap<String, Learning>
}

impl UserLearning {
  pub fn new(db: WordDb) -> Self {
    let words = db.map().iter().map(|(word, _)| (word.clone(), Learning::default())).collect();
    Self {
      words
    }
  }

  pub fn update(&mut self, words: Vec<(&String, LearningStatus)>) {
    for (word, status) in words {
      if let Some(learning) = self.words.get_mut(word) {
        match status {
          LearningStatus::Search => {
            learning.searched += 1;
            learning.seen += 1;
            if learning.acc > 0 {
              learning.acc = -1;
            } else {
              learning.acc -= 1;
            }
          },
          LearningStatus::NoSearch => {
            learning.seen += 1;
            learning.acc += 1;
          }
        }
      }
    }
  }
}
