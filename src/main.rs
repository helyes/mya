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
    directory: Option<String>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Snippet {
    Commands(BTreeMap<String, Details>),
}

#[derive(Debug)]
enum Action {
  // Store the yaml key and additional parameters
  Run(String, Vec<String>),
  Help,
  List(ActionListMode),
  Unknown
}

#[derive(Debug)]
enum ActionListMode {
  Short, // this is for bash code completition
  Verbose
}

fn parse_config(args: &[String]) -> Action {
  debug!("Parsing args {:?}", args);
  let mut ret: Action = Action::Unknown;
  match args {
    [only_one] => { 
      debug!("help for {}", only_one);
      ret = Action::Help;
      return ret;
    },
    [_first, action] => {
      match action.as_str() {
        "help" => return Action::Help,
        "list" => return Action::List(ActionListMode::Verbose),
        _ => println!("{}: {}", "Wrong parameter".red(), action.as_str().red())
      }

    },
    _ => println!("{} {}", "Too many parameters:".red(), args.len().to_string().red()),
  }
   
  match args[1].as_str() {
    "run" => ret = Action::Run(String::from(args[2].as_str()), Vec::from(args)),
    "list" => ret = Action::List(ActionListMode::Short), // whatever comes after list will result in short list
    _ => error!("NOT a Run or List action")
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

fn get_snippet_details(snippet_key: &str) -> Details { 
  debug!("Getting snippet {} details...", &snippet_key);
  let snippets: Snippet = read_snippets();
  let available_snippets: BTreeMap<String, Details>;
  match snippets {
    Snippet::Commands(value) => {
      available_snippets = value;
    }
  }
  
  debug!("Snippet for '{}': {:?}", snippet_key , available_snippets.get(snippet_key));
  let snippet_details: &Details;
  match available_snippets.get(snippet_key) {
    None => {
      println!("{}", "Snippet does not exist".red());
      process::exit(1);
    },
    _ => snippet_details = available_snippets.get(snippet_key).unwrap()
  }

  let rr = Details {
    command: snippet_details.command.to_owned(),
    description: snippet_details.description.to_owned(),
    directory: snippet_details.directory.to_owned()

  };
  return rr;

}

fn populate_command_placeholders(command: &str, args: &[String]) -> String {
  debug!("  ARGS: {:?}", args);
  let mut ret = String::from(command).as_str().to_owned();
  for i in 0..args.len() {
    println!("FOR: {}", args[i]);
    let placeholder = format!("{{{}}}", i+1);
    debug!("  Replacing {} to {}", placeholder,  args[i]);
    ret = ret.replace(&placeholder, &args[i]);
    debug!("  Replaced {}", ret);
  }
  return String::from(ret);
}

fn run_snippet(snippet_key: &str, args: &[String]) {
  
  debug!("Running {} snippet, args: {:?}", &snippet_key, args);

  let mut snippet_details = get_snippet_details(snippet_key);
  debug!("  Command: {}", snippet_details.command);
  debug!("  Description: {}", snippet_details.description);
  match &snippet_details.directory {
    Some(dir) => debug!("  Directory: {}", dir),
    None      => debug!("  No directory configured"),
  }

  let command_with_palceholders = populate_command_placeholders(&snippet_details.command, &args[3..]);
  debug!("  Command PPP: {}", command_with_palceholders);

  snippet_details.command = command_with_palceholders;
  let output = execute(&snippet_details, args);

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
    Action::Run(snippet_key, arga) =>  {
      run_snippet(&snippet_key, &arga)//&args);
    }
    _ => {
      println!("Action not implemented: {:?}", a);
      process::exit(3);
    }
  };

}

fn get_placeholders_mapping(d: &Vec<String>) -> BTreeMap<String, String> { 
  let mut ret = BTreeMap::new();
  debug!("Getting placeholders..."); 
  ret.insert(String::from("{{1}}"), String::from("snippets"));
  return ret;
}

fn execute(d: &Details, args: &[String]) -> std::process::Output {
  let command_executable= get_command_without_parameters(&d.command);

  debug!("  Executable: {}", command_executable);
  let arguments = get_arguments(&d.command);
  debug!("  Arguments: {:?}", arguments);

  let placeholder_map = get_placeholders_mapping(&arguments);
  debug!("  Placeholder map: {:?}", placeholder_map);
  
  let mut command = Command::new(command_executable);

  // add arguments
  for argument in arguments { 
    // replace {{1}} and so on...
    if argument == "{{1}}" {
      let replaced_placeholder = &args[3];
      debug!("  Adding argument:: {}", replaced_placeholder);
      command.arg(replaced_placeholder);
    } else {
      debug!("  Adding argument:: {}", argument);
      command.arg(argument);
    }
  };
  let a = String::from("No dir!");
  let directory = d.directory.as_ref().unwrap_or(&a);
  let expanded_dir = shellexpand::full(&directory).unwrap();
  if directory != &a {
    debug!("  Adding dir: {:?}", expanded_dir);
    command.current_dir(expanded_dir.as_ref());
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
    // skip firs argument. That's the script name
    if i!= 0 {
      debug!("Found argument: {}", &cap[0]);
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
