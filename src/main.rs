// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// use colored::*;
use std::env;
use std::process;

#[macro_use]
extern crate log;

mod action;
use crate::action::{list, run};
mod snippet;
use crate::snippet::{loader, model::Snippet};
mod util;
extern crate clap;

use clap::{App, AppSettings, Arg};

fn main() {
  env_logger::init();
  // mut as it needs to be shifted if group is used
  let mut args_to_pass: Vec<String> = env::args().collect();
  let exit_code: i32;
  // https://github.com/brocode/fw/blob/c803cd20dd6f5f4f07fb4c061f7ac9e8240ea29b/src/app/mod.rs
  let matches = App::new("mya")
    .about("My better aliases")
    .version("1.0")
    .global_setting(AppSettings::ColoredHelp)
    .setting(AppSettings::SubcommandRequired)
    .author("Andras <helyesa@gamil.com>")
    // list takes an optional -s flag
    .subcommand(
      App::new("list")
        .about("Lists available aliases")
        .arg(
          Arg::with_name("alias")
            .help("Alias to run")
            .required(false)
            .multiple(true),
        )
        .arg(Arg::with_name("short").short("s").help("short list")),
    )
    .subcommand(
      App::new("run")
        .about("Runs given alias")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(
          Arg::with_name("alias")
            .help("Alias to run")
            .required(true)
            .multiple(true),
        ),
    )
    .get_matches();
  match matches.subcommand() {
    ("run", Some(add_matches)) => {
      exit_code = handle_run(add_matches);
    }
    ("list", Some(list_matches)) => {
      exit_code = handle_list(list_matches);
    }
    ("", None) => {
      println!("No subcommand was used"); // this can't happen as subcommands are mandatory
      exit_code = 7;
    } // If no subcommand was used it'll match the tuple ("", None)
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
  }

  process::exit(exit_code);
}

fn handle_list(list_matches: &clap::ArgMatches<'_>) -> i32 {
  let exit_code: i32;
  let group_names = loader::get_group_names();
  let run_params_options = list_matches.values_of("alias");
  let run_params :Vec<_>;

  match run_params_options {
    Some(options) => {
      println!("Option given. {:?}", options);
      run_params = options.collect::<Vec<_>>()
    }
    None =>  {
      println!("No option was given");
      // TODO: refactor
      run_params = Vec::new();
    }
  }
  let snippet_group: Option<&String>;
  match run_params.len() {
    0 => {
      // no group given
      // "$ mya list" was run
      println!("No group set");
      snippet_group = None;
    }
    _ => {
      // group set - only care about first one
      // "$ mya list shell" was run
      snippet_group = group_names.get(run_params[0]);
    }
  };

  println!("Group: {:?}", snippet_group);

  if list_matches.is_present("short") {
    println!("List short");
    // "$ mya list -s" was run
    // let snippets: Snippet = loader::read_snippets(&None);
    exit_code = list::execute(list::ActionListMode::Short, snippet_group);
    
    // exit_code = 1;
  } else {
    println!("List long");
    // "$ mya list" was run
    exit_code = list::execute(list::ActionListMode::Verbose, snippet_group);
  }
  return exit_code;
}

fn handle_run(add_matches: &clap::ArgMatches<'_>) -> i32 {
  let exit_code: i32;
  let mut args_to_pass: Vec<String> = env::args().collect();
  let group_names = loader::get_group_names();

  // Reference to run's matches
  // println!("Running alias {:?}", add_matches.values_of("alias").unwrap().collect::<Vec<_>>().join(", "));
  let run_params = add_matches.values_of("alias").unwrap().collect::<Vec<_>>();
  let snippet_group: Option<&String>;
  let snippet_key: &str;
  match run_params.len() {
    0 => {
      // this scenario can't happen due to clap config
      panic!("Run subcommand must be followed by alias name")
    }
    1 => {
      // no group: mya run taskname was run
      snippet_group = None;
      snippet_key = run_params[0];
    }
    _ => {
      // figure if second parameter is a group
      snippet_group = group_names.get(run_params[0]);
      match snippet_group {
        Some(_group) => {
          snippet_key = run_params[1];
          &args_to_pass.remove(0);
        }
        None => snippet_key = run_params[0],
      }
    }
  };

  let snippets: Option<Snippet> = loader::get_snippet_for_key(&snippet_group, &snippet_key);
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
