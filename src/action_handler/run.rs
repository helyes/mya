use std::env;
use crate::action::run;
use crate::snippet::{loader, model::Snippet};

pub fn handle(run_matches: &clap::ArgMatches<'_>) -> i32 {
  let exit_code: i32;
  let mut args_to_pass: Vec<String> = env::args().collect();
  let available_groups = loader::get_group_names();
  // Reference to run's matches
  // println!("Running alias {:?}", add_matches.values_of("alias").unwrap().collect::<Vec<_>>().join(", "));
  let run_params = run_matches.values_of("alias").unwrap().collect::<Vec<_>>();
  let group: Option<&String>;
  let snippet_key: &str;
  match run_params.len() {
    0 => {
      // this scenario can't happen due to clap config
      panic!("Run subcommand must be followed by alias name")
    }
    1 => {
      // no group: mya run taskname was run
      group = None;
      snippet_key = run_params[0];
    }
    _ => {
      // figure if second parameter is a group
      group = available_groups.get(run_params[0]);
      match group {
        Some(_group) => {
          snippet_key = run_params[1];
          &args_to_pass.remove(0);
        }
        None => snippet_key = run_params[0],
      }
    }
  };

  let snippets: Option<Snippet> = loader::get_snippet_for_key(&group, &snippet_key);
  match snippets {
    // snippet found for key
    Some(snippet) => {
      let snippet_details = snippet.get_details_for(&snippet_key);
      match snippet_details {
        Some(details) => {
          exit_code = run::execute(&snippet_key, details, &args_to_pass);
        }
        None => {
          println!("Could not find details for snippet {:?}", snippet);
          exit_code = 4
        }
      }
    }
    None => {
      println!("Could not find snippet for key {:?}", &snippet_key);
      exit_code = 5
    }
  }

  return exit_code;
}

// "([^"]+)"|\s*([^"\s]+)
