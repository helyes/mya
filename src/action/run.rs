use colored::*;
use std::process::Command;
use std::str;

use crate::snippet::model::{ Details };
use crate::util::string_util;

pub fn execute(snippet_key: &str, mut snippet_details: Details, args: &[String]) -> i32 {
  debug!("Running {} snippet, args: {:?}", &snippet_key, args);

  // let mut snippet_details = snippets.get_details_for(snippet_key);
  debug!("Command: {}", snippet_details.command);
  debug!("Description: {}", snippet_details.description);
  match &snippet_details.directory {
    Some(dir) => debug!("Directory: {}", dir),
    None      => debug!("No directory configured"),
  }
  
  snippet_details.command = string_util::personalize(&snippet_details.command, &args[3..]);
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

fn execute_snippet_details(d: &Details) -> std::process::Output {
  debug!("Preparing to execute command entry: {}", d.command);

  let command_executable= &d.get_command_without_parameters();
  let arguments = &d.get_command_arguments();
  
  let mut command = Command::new(&command_executable);

  // add arguments
  for argument in arguments {
      debug!("Adding argument to '{}' command: {}", &command_executable, &argument);
      command.arg(argument);
  };

  let a = String::from("No dir!");
  let directory = d.directory.as_ref().unwrap_or(&a);
  let expanded_dir = shellexpand::full(&directory).unwrap();
  if directory != &a {
    debug!("Adding dir: {:?}", expanded_dir);
    command.current_dir(expanded_dir.as_ref());
  }

  debug!("Executing '{}' with parameters: {:?}", &command_executable, &arguments);

  let output = command.output().expect("failed to execute process");
  return output;
}
