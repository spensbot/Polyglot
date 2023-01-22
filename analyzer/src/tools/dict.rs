use std::collections::{HashMap};

const PATH: &str = "../texts/english/dictionary/dictionary.json";

pub struct Dict {
  definitions: HashMap<String, String>
}

impl Dict {
  pub fn load() -> Self {
    let dict_string = std::fs::read_to_string(PATH).unwrap();

    let definitions: HashMap<String, String> = serde_json::from_str(&dict_string).unwrap();
    // let words = definitions.iter().map(|(word, _)| word.clone()).collect();

    Self {
      definitions
    }
  }

  pub fn into_definitions(self) -> HashMap<String, String> {
    self.definitions
  }
}