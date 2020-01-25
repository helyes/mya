use std::env;
use std::fs;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
// https://serde.rs/derive.html
// https://github.com/dtolnay/serde-yaml
// https://stackoverflow.com/questions/55245914/how-to-use-serde-to-parse-a-yaml-file-with-multiple-different-types
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Details {
    pub command: String,
    pub description: String,
    pub directory: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Snippet {
    Commands(BTreeMap<String, Details>),
}


pub fn load_file() {
  println!("from loader");
}

pub fn get_snipet_file_path() -> String {
  let file_path = String::from("snippets/shell.yaml"); // some_string comes into scope
  file_path                              
}

pub fn read_snippets() -> Snippet {

  let key = "HOME";
  // let mut snippets_home = ""
  match env::var_os(key) {
    Some(val) => println!("VARIABLE FOUND: {}: {:?}", key, val),
    None => println!("{} is not defined in the environment.", key)
  }
  let file_to_read = get_snipet_file_path();
  debug!("Reading file: {}", file_to_read);

  let contents = fs::read_to_string(file_to_read)
    .expect("Something went wrong reading the file");
  let snippets: Snippet = serde_yaml::from_str(&contents).unwrap();
  return snippets;
}