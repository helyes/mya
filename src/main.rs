use std::env;
use std::fs;

use serde::{Serialize, Deserialize};
//use std::collections::BTreeMap;
use std::collections::HashMap;

// https://serde.rs/derive.html
// https://github.com/dtolnay/serde-yaml
// https://stackoverflow.com/questions/55245914/how-to-use-serde-to-parse-a-yaml-file-with-multiple-different-types
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Details {
    command: String,
    description: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Snippet {
    Commands(HashMap<String, Details>),
}

fn main() {

  let args: Vec<String> = env::args().collect();

  let (action, key) = parse_config(&args);
  println!("Action {}, key: {}", action, key);

  let file_to_read = get_snipet_file_path();
  println!("Reading file: {}", file_to_read);

  let contents = fs::read_to_string(file_to_read)
    .expect("Something went wrong reading the file");
  let snippets: Snippet = serde_yaml::from_str(&contents).unwrap();
  println!("{:?}", snippets);

  let available_snippets: HashMap<String, Details>;
  match snippets {
    Snippet::Commands(value) => {
      println!("value: {:?}", value);
      available_snippets = value;
    }
  }

  println!("Snippet for '{}': {:?}", key , available_snippets.get(key));
  let details_for_command: &Details = available_snippets.get(key).unwrap();
  println!("  Command: {}", details_for_command.command);
  println!("  Description: {}", details_for_command.description);

}

fn get_snipet_file_path() -> String {
  let file_path = String::from("snippets/shell.yaml"); // some_string comes into scope
  file_path                              
}

fn parse_config(args: &[String]) -> (&str, &str) {
  let action = &args[1];
  let key = &args[2];

  (action, key)
}

  // let content_hash = r#"
  // {
  //       "ONE_ENV": 
  //         {command : "fake" }
  // }
  // "#;
  // let snippet_data: Snippet = serde_yaml::from_str(content_hash).unwrap();
  // println!("{:?}", snippet_data);
  

  // match serde_yaml::to_string(&snippet_data) {
  //     Ok(s1) => println!("TOSTRING{:?}", s1), 
  //     Err(_) => { println!("Error") },
  // };
