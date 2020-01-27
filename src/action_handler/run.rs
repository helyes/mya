use colored::*;
use std::env;
use crate::action::run;
use crate::alias::{loader, model::Alias};

pub fn handle(run_matches: &clap::ArgMatches<'_>) -> i32 {
  let exit_code: i32;
  let mut args_to_pass: Vec<String> = env::args().collect();
  let available_groups = loader::get_group_names();
  // Reference to run's matches
  // println!("Running alias {:?}", add_matches.values_of("alias").unwrap().collect::<Vec<_>>().join(", "));
  let run_params = run_matches.values_of("alias").unwrap().collect::<Vec<_>>();
  let group: Option<&String>;
  let alias_key: &str;
  match run_params.len() {
    0 => {
      // this scenario can't happen due to clap config
      panic!("Run subcommand must be followed by alias name")
    }
    1 => {
      // no group: mya run taskname was run
      group = None;
      alias_key = run_params[0];
    }
    _ => {
      // figure if second parameter is a group
      group = available_groups.get(run_params[0]);
      match group {
        Some(_group) => {
          alias_key = run_params[1];
          &args_to_pass.remove(0);
        }
        None => alias_key = run_params[0],
      }
    }
  };

  let aliases: Option<Alias> = loader::get_alias_for_key(&group, &alias_key);
  match aliases {
    // alias found for key
    Some(alias) => {
      let alias_details = alias.get_details_for(&alias_key);
      match alias_details {
        Some(details) => {
          exit_code = run::execute(&alias_key, details, &args_to_pass);
        }
        None => {
          println!("Could not find details for alias {:?}", alias);
          exit_code = 4
        }
      }
    }
    None => {
      // println!("Could not find alias for key {:?} in below groups:\n", &alias_key);
      println!("\n{} '{}' {}", "Could not find alias for key".yellow(), &alias_key.red(), "in below groups:\n".yellow());
      for (key, value) in &available_groups {
        println!("{}: \t{}", key.bold(), value);
      }
      exit_code = 5
    }
  }

  return exit_code;
}

// "([^"]+)"|\s*([^"\s]+)
