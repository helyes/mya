// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// use colored::*;
use std::env;
use std::process;

#[macro_use] extern crate log;

mod action;
use crate::action::{ help, list, run };
mod snippet;
use crate::snippet::{ loader, model::Snippet };
mod util;

#[derive(Debug)]
enum Action {
  Help,
  List(list::ActionListMode),
  // Store the yaml key and additional parameters
  Run(String, Vec<String>),
}

fn parse_arguments(args: &[String]) -> Result<Action, &str> {
  debug!("Parsing args {:?}", args);
  match args {
    [only_one] => { 
      debug!("help for {}", only_one);
      return Ok(Action::Help);
    },
    [_first, action] => {
      match action.as_str() {
        "help" => return Ok(Action::Help),
        "list" => return Ok(Action::List(list::ActionListMode::Verbose)),
        _ => return Err("Wrong parameter, only list and help works without parameter")
      }

    },
    _ => () //println!("{} {}", "Too many parameters:".red(), args.len().to_string().red()),
  }
   
  match args[1].as_str() {
    "run" => {
      if args.len() < 3 {
        return Err("Run command must be followed by target defined in yaml")
      }
      return Ok(Action::Run(String::from(args[2].as_str()), Vec::from(args)))
    },
    "list" => return Ok(Action::List(list::ActionListMode::Short)), // whatever comes after list will result in short list
    _ => return Err("NOT a Run or List action. Action not implemented")
  }
}

fn main() {
  env_logger::init();
  let args: Vec<String> = env::args().collect();
  let exit_code: i32;
  let action_result = parse_arguments(&args);
  let action : Action;

  match action_result {
    Ok(val) => action = val,
    Err(message) => {
      println!("Error: {:?}", message);
      process::exit(3);
    }
  }

  match action {
    Action::Help =>  {
      exit_code = help::execute();
    },
    Action::List(l) =>  {
      let snippets: Snippet = loader::read_snippets();
      exit_code = list::execute(l, snippets);
    }
    Action::Run(snippet_key, arga) =>  {
      let snippets: Snippet = loader::read_snippets();
      let snippet_details = snippets.get_details_for(&snippet_key);
      match snippet_details {
        Some(details) => {
          exit_code = run::execute(&snippet_key, details, &arga); 
        }
        None => exit_code = 4
      }
    }
  };
  process::exit(exit_code);
}
// "([^"]+)"|\s*([^"\s]+)
