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
  let a= d.command.clone();
  let b= get_command_without_parameters(a);

  println!("  THE a: {}", b);
  let output = Command::new(b)
  .arg("-l")
  //.arg("echo hello")
  .output()
  .expect("failed to execute process");
  let ret = str::from_utf8(&output.stdout).unwrap();
  return String::from(ret);
}

fn get_command_without_parameters (command: String) -> String{
  // let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
  let re = Regex::new(r#"^"(.*?)".*"#).unwrap();
  //re.is_match(&command);
  // println!("  MAtch1:{}", re.is_match(&command));
  let mut ret: String = String::from("");
  for cap in re.captures_iter(&command) {
    //println!("Month: {}", &cap[1]);
    ret =  String::from(&cap[1]);
  }

  if ret.is_empty() {
    let re = Regex::new(r"^(.*?)\s").unwrap();
    // println!("  MAtch2:{}", re.is_match(&command));
    for cap in re.captures_iter(&command) {
      //println!("Month2: {}", &cap[1]);
      ret =  String::from(&cap[1]);
    }
  
  }

  return ret;
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
