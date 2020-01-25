use colored::*;
use regex::Regex;
use std::collections::BTreeMap;
use std::process::Command;
use std::process;
use std::str;

use crate::snippet::model::{ Snippet, Details};

pub fn execute(snippet_key: &str, snippets: Snippet, args: &[String]) -> i32 {
  debug!("Running {} snippet, args: {:?}", &snippet_key, args);

  let mut snippet_details = get_snippet_details(snippet_key, snippets);
  debug!("  Command: {}", snippet_details.command);
  debug!("  Description: {}", snippet_details.description);
  match &snippet_details.directory {
    Some(dir) => debug!("  Directory: {}", dir),
    None      => debug!("  No directory configured"),
  }

  let command_with_palceholders = populate_command_placeholders(&snippet_details.command, &args[3..]);

  snippet_details.command = command_with_palceholders;
  let output = execute_snippet_details(&snippet_details);

  match output.status.code() {
      Some(code) => {
        debug!("  Output status:{}", output.status.code().unwrap());
        if !&output.stdout.is_empty() {
          println!("\n{}", str::from_utf8(&output.stdout).unwrap());
        }
        if !&output.stderr.is_empty() {
          println!("\n{}", str::from_utf8(&output.stderr).unwrap().red());
        }
        return code;
      },
      None => {
        println!("{}", "Process terminated by signal. Unknow exit code".red());
        return 255;
      }
    }
}

fn get_arguments (command_with_params: &String) -> Vec<String> { 
  // ideal regex, not supported: "(.+?)"|([-\w]+(?=\s|$))
  let re = Regex::new(r#"("[^"]+")|\S+"#).unwrap();
  let mut vec = Vec::new();
  debug!("Getting arguments for command...");
  let mut i: i16 = 0;
  for cap in re.captures_iter(command_with_params) {
    // skip firs argument. That's the script name
    if i!= 0 {
      // debug!("  Found argument: {}", &cap[0]);
      vec.push(String::from(&cap[0]));
    }
    i = i+1;
  }
  debug!("  Arguments: {:?}", vec);
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
  debug!("  Command to execute: {}", &ret);
  return String::from(ret);
}

fn populate_command_placeholders(command: &str, args: &[String]) -> String {
  debug!("Populating command placeholders. args: {:?}", args);
  let mut ret = String::from(command).as_str().to_owned();
  for i in 0..args.len() {
    let placeholder = format!("{{{}}}", i+1);
    debug!("  Replacing {} to {}", placeholder,  args[i]);
    ret = ret.replace(&placeholder, &args[i]);
  }

  //expand env variables and ~
  let ret_expanded = shellexpand::full(&ret).unwrap();
  debug!("  Replaced command: {}", ret_expanded);
  return String::from(ret_expanded);
}


fn execute_snippet_details(d: &Details) -> std::process::Output {
  debug!("Preparing to execute comand entry: {}", d.command);

  let command_executable= get_command_without_parameters(&d.command);
  let arguments = get_arguments(&d.command);
  
  let mut command = Command::new(&command_executable);

  // add arguments
  for argument in &arguments {
      debug!("  Adding argument to '{}' command: {}", &command_executable, &argument);
      command.arg(argument);
  };

  let a = String::from("No dir!");
  let directory = d.directory.as_ref().unwrap_or(&a);
  let expanded_dir = shellexpand::full(&directory).unwrap();
  if directory != &a {
    debug!("  Adding dir: {:?}", expanded_dir);
    command.current_dir(expanded_dir.as_ref());
  }

  debug!("Executing '{}' with parameters: {:?}", &command_executable, &arguments);

  let output = command.output().expect("failed to execute process");
  return output;
}

fn get_snippet_details(snippet_key: &str, snippets: Snippet) -> Details { 
  debug!("Getting snippet {} details...", &snippet_key.green());
  // let snippets: Snippet = loader::read_snippets();
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
      println!("{}", "\nSnippet does not exist".red());
      println!("{}", "Run list to see available snippets");
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