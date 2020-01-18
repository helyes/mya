// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;
use std::str;

use regex::Regex;
use serde::{Serialize, Deserialize};


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
  // println!("{:?}", snippets);

  let available_snippets: HashMap<String, Details>;
  match snippets {
    Snippet::Commands(value) => {
      // println!("value: {:?}", value);
      available_snippets = value;
    }
  }

  println!("Snippet for '{}': {:?}", key , available_snippets.get(key));
  let details_for_command: &Details = available_snippets.get(key).unwrap();
  println!("  Command: {}", details_for_command.command);
  println!("  Description: {}", details_for_command.description);

  let dcopy = details_for_command.clone();
  let out = execute(dcopy);
 // println!("  Out: {}", str::from_utf8(&out).unwrap());
  println!("  Out:\n\n{}", out);
}

fn execute(d: &Details) -> String {
  let command_with_params= d.command.clone();
  let command_executable= get_command_without_parameters(command_with_params);

  println!("  THE a: {}", command_executable);
  let output = Command::new(command_executable)
  .arg("-l")
  .arg("-h")
  .output()
  .expect("failed to execute process");
  let ret = str::from_utf8(&output.stdout).unwrap();
  return String::from(ret);
}

fn get_command_without_parameters (command: String) -> String {

  if !command.trim().contains(" ") {
    return command;
  }

  let re = Regex::new(r#"^"(.*?)".*"#).unwrap();
  let mut captures = re.captures(&command);  
  let mut ret;
  match captures {
    None => ret = "",
    _ => ret = captures.unwrap().get(1).map_or("", |m| m.as_str()),
  }
  if ret.is_empty() {
    let re = Regex::new(r"^(.*?)\s").unwrap();
    captures = re.captures(&command);
    ret = captures.unwrap().get(1).map_or("", |m| m.as_str());
  }
  return String::from(ret);
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
