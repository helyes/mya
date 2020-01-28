use colored::*;
use std::process::Command;
use std::str;

use crate::alias::model::Details;
use crate::util::string_util;

pub fn execute(alias_key: &str, mut alias_details: Details, args: &[String]) -> i32 {
  debug!("Running {} alias, args: {:?}", &alias_key, args);

  // let mut alias_details = aliases.get_details_for(alias_key);
  debug!("Command: {}", alias_details.command);
  debug!("Description: {}", alias_details.description);
  match &alias_details.directory {
    Some(dir) => debug!("Directory: {}", dir),
    None => debug!("No directory configured"),
  }
  match string_util::personalize(&alias_details.command, &args[3..]) {
    Ok(personalized) => alias_details.command = personalized,
    Err(msg) => {
      println!("{}", msg.red());
      return 9;
    }
  }
  //alias_details.command = string_util::personalize(&alias_details.command, &args[3..]);
  let output = execute_alias_details(&alias_details);

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
    }
    None => {
      println!("{}", "Process terminated by signal. Unknow exit code".red());
      return 255;
    }
  }
}

fn execute_alias_details(d: &Details) -> std::process::Output {
  debug!("Preparing to execute command entry: {}", d.command);

  let command_executable = &d.get_command_without_parameters();
  let arguments = &d.get_command_arguments();
  let mut command = Command::new(&command_executable);

  // add arguments
  for argument in arguments {
    debug!(
      "Adding argument to '{}' command: {}",
      &command_executable, &argument
    );
    command.arg(argument);
  }

  let a = String::from("No dir!");
  let directory = d.directory.as_ref().unwrap_or(&a);
  let expanded_dir = shellexpand::full(&directory).unwrap();
  if directory != &a {
    debug!("Adding dir: {:?}", expanded_dir);
    command.current_dir(expanded_dir.as_ref());
  }

  debug!(
    "Executing '{}' with parameters: {:?}",
    &command_executable, &arguments
  );
  println!("\n{}", d.description.green().bold());
  println!("{}\n",  string_util::underscore_for(&d.description));
  if directory != &a {
    println!("{} {}", "cd".yellow(), expanded_dir.as_ref().yellow());
  }
  println!("{}\n", d.command.yellow());
  let output = command.output().expect(&format!("Failed to execute: {}", d.description));
  return output;
}
