// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;
use std::process;
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
  let details_for_command: &Details;
  match available_snippets.get(key) {
    None => {
      println!("Snippet does not exist");
      process::exit(1);
    },
    _ => details_for_command = available_snippets.get(key).unwrap()
  }

  println!("  Command: {}", details_for_command.command);
  println!("  Description: {}", details_for_command.description);

  let dcopy = details_for_command.clone();
  let output = execute(dcopy);

  match output.status.code() {
      Some(code) => {
        println!("  Output status:{}", output.status.code().unwrap());
        if !&output.stdout.is_empty() {
          println!("  Std Out:\n{}", str::from_utf8(&output.stdout).unwrap());
        }
        if !&output.stderr.is_empty() {
          println!("  Std Err:\n{}", str::from_utf8(&output.stderr).unwrap());
        }
        process::exit(code);
      },
      None => {
        println!("  Process terminated by signal. Unknow exit code.");
        process::exit(255)
      }
    }

}

fn execute(d: &Details) -> std::process::Output {
  let command_with_params= d.command.clone();
  let command_executable= get_command_without_parameters(&command_with_params);

  println!("  Executable: {}", command_executable);
  let arguments = get_arguments(&command_with_params);
  println!("  Arguments: {:?}", arguments);
  let mut command = Command::new(command_executable);
  
  // add arguments
  for argument in arguments {
    println!("ARG: {}", argument);
    command.arg(argument);
  }
  
  let output = command.output().expect("failed to execute process");
  return output;
}

fn get_arguments (command_with_params: &String) -> Vec<String> { 
  // ideal regex, not supported: "(.+?)"|([-\w]+(?=\s|$))
  let re = Regex::new(r#"("[^"]+")|\S+"#).unwrap();
  let mut vec = Vec::new();
  println!("Getting arguments...");
  let mut i: i32 = 0;
  for cap in re.captures_iter(command_with_params) {
    if i!= 0 {
      println!("Found param: {}", &cap[0]);
      vec.push(String::from(&cap[0]));
    }
    i = i+1;
  }
  
  // vec.push(String::from("-l"));
  // vec.push(String::from("-h"));
  return vec;
}



fn get_command_without_parameters (command: &String) -> String {
  println!("Getting command without parameters...");
  if !command.trim().contains(" ") {
    return String::from(command);
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
// "([^"]+)"|\s*([^"\s]+)


fn get_command_without_parameters_old (command: String) -> String {

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
