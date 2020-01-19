// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
use colored::*;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::process::Command;
use std::process;
use std::str;

use regex::Regex;
use serde::{Serialize, Deserialize};

#[macro_use] extern crate log;


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
    Commands(BTreeMap<String, Details>),
}

#[derive(Debug)]
enum Action {
  Run(String),
  Help,
  List(ActionListMode),
  Unknown
}

#[derive(Debug)]
enum ActionListMode {
  Short, // this is for bash code completition
  Verbose
}

// #[derive(Debug)]
// struct Executable {
//   action: Action,
//   key: String,
// }


fn parse_config(args: &[String]) -> Action {
  debug!("Parsing args {:?}", args);
  let mut ret: Action = Action::Unknown;
  match args {
    [only_one] => { 
      debug!("help for {}", only_one);
      ret = Action::Help;
    },
    [_first, action] => {
      match action.as_str() {
        "help" => ret = Action::Help,
        "list" => ret = Action::List(ActionListMode::Verbose),
        _ => println!("{}: {}", "Wrong parameter".red(), action.as_str().red())
      }

    },
    [_first, action, param] => { 
      debug!("Params: {} and {}",action, param);
      match action.as_str() {
        "run" => ret = Action::Run(String::from(param)),
        "list" => ret = Action::List(ActionListMode::Short), // whatever comes after list will result in short list
        _ => error!("NOT a Run or List action")
      }
      // println!("!!!!!!!!!Acrion enum: {:?}", &ret);
    },
    _ => println!("{} {}", "Too many parameters:".red(), args.len().to_string().red()),
  }

  return ret;
}

fn read_snippets() -> Snippet {
  let file_to_read = get_snipet_file_path();
  debug!("Reading file: {}", file_to_read);

  let contents = fs::read_to_string(file_to_read)
    .expect("Something went wrong reading the file");
  let snippets: Snippet = serde_yaml::from_str(&contents).unwrap();
  return snippets;
}


fn left_pad(s: &str, pad: usize, padchar: char) -> String
{
    let mut ret = String::new();
    for _ in 0..pad {
        ret.push(padchar);
    }
    ret.push_str(s);
    ret
}

fn list(mode: ActionListMode) {
  debug!("Listing snippets");
  let snippets: Snippet = read_snippets();
  debug!("Snippets: {:?}", snippets);
  let available_snippets: BTreeMap<String, Details>;
  match snippets {
    Snippet::Commands(value) => {
      // println!("value: {:?}", value);
      available_snippets = value;
    }
  }
  //println!("Available Snippets: {:?}", available_snippets);

  match mode {
    ActionListMode::Short => {
      for (key, _value) in &available_snippets {
        print!("{} ", key);
      }
    },
    ActionListMode::Verbose => {
      println!("\n{}", "Available commands".green().bold());
      let mut longest_key_length = 0;
      for (key, _value) in &available_snippets {
        if key.len() > longest_key_length {
          longest_key_length = key.len();
        }
      }
      for (key, value) in &available_snippets {
        let keyp = format!("  {}", key);
        println!("{} {}", keyp.yellow().bold(), left_pad(&value.command, longest_key_length - key.len(), ' ').bold());
        println!("{}", left_pad(&value.description, longest_key_length + 3, ' '));
      }
    }
  }
}


fn run(key: &str) {
  debug!("RUNNING {}", &key);

  let snippets: Snippet = read_snippets();
  let available_snippets: BTreeMap<String, Details>;
  match snippets {
    Snippet::Commands(value) => {
      // println!("value: {:?}", value);
      available_snippets = value;
    }
  }


  debug!("Snippet for '{}': {:?}", key , available_snippets.get(key));
  let details_for_command: &Details;
  match available_snippets.get(key) {
    None => {
      error!("Snippet does not exist");
      process::exit(1);
    },
    _ => details_for_command = available_snippets.get(key).unwrap()
  }

  debug!("  Command: {}", details_for_command.command);
  debug!("  Description: {}", details_for_command.description);

  let dcopy = details_for_command.clone();
  let output = execute(dcopy);

  match output.status.code() {
      Some(code) => {
        debug!("  Output status:{}", output.status.code().unwrap());
        if !&output.stdout.is_empty() {
          println!("\n{}", str::from_utf8(&output.stdout).unwrap());
        }
        if !&output.stderr.is_empty() {
          println!("\n{}", str::from_utf8(&output.stderr).unwrap().red());
        }
        process::exit(code);
      },
      None => {
        println!("{}", "Process terminated by signal. Unknow exit code".red());
        process::exit(255)
      }
    }
}

fn main() {
  env_logger::init();
  let args: Vec<String> = env::args().collect();
  let a = parse_config(&args);

  match a {
    Action::Help =>  {
      debug!("Printing help");
      process::exit(0);
    },
    Action::List(l) =>  {
      list(l);
    }
    Action::Run(key) =>  {
      run(&key);
    }
    _ => {
      println!("Action not implemented: {:?}", a);
      process::exit(3);
    }
  };

}

fn execute(d: &Details) -> std::process::Output {
  let command_executable= get_command_without_parameters(&d.command);

  debug!("  Executable: {}", command_executable);
  let arguments = get_arguments(&d.command);
  debug!("  Arguments: {:?}", arguments);
  let mut command = Command::new(command_executable);
  
  // add arguments
  for argument in arguments {
    debug!("Adding argument:: {}", argument);
    command.arg(argument);
  }
  
  let output = command.output().expect("failed to execute process");
  return output;
}

fn get_arguments (command_with_params: &String) -> Vec<String> { 
  // ideal regex, not supported: "(.+?)"|([-\w]+(?=\s|$))
  let re = Regex::new(r#"("[^"]+")|\S+"#).unwrap();
  let mut vec = Vec::new();
  debug!("Getting arguments...");
  let mut i: i16 = 0;
  for cap in re.captures_iter(command_with_params) {
    if i!= 0 {
      debug!("Found param: {}", &cap[0]);
      vec.push(String::from(&cap[0]));
    }
    i = i+1;
  }
  
  return vec;
}


fn get_command_without_parameters (command: &String) -> String {
  debug!("Getting command without parameters...");
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

fn get_snipet_file_path() -> String {
  let file_path = String::from("snippets/shell.yaml"); // some_string comes into scope
  file_path                              
}
