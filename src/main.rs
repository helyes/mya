// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
use colored::*;
use std::env;
use std::process;

#[macro_use] extern crate log;

mod snippet;
use crate::snippet::{ loader, model::Snippet };
mod action;
use crate::action::{ help, list, run };
mod util;

#[derive(Debug)]
enum Action {
  // Store the yaml key and additional parameters
  Run(String, Vec<String>),
  Help,
  List(list::ActionListMode),
  Unknown
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
        "list" => return Action::List(list::ActionListMode::Verbose),
        _ => println!("{}: {}", "Wrong parameter".red(), action.as_str().red())
      }

    },
    _ => () //println!("{} {}", "Too many parameters:".red(), args.len().to_string().red()),
  }
   
  match args[1].as_str() {
    "run" => ret = Action::Run(String::from(args[2].as_str()), Vec::from(args)),
    "list" => ret = Action::List(list::ActionListMode::Short), // whatever comes after list will result in short list
    _ => error!("NOT a Run or List action")
  }

  return ret;
}

fn main() {
  env_logger::init();
  let args: Vec<String> = env::args().collect();
  let a = parse_config(&args);

  match a {
    Action::Help =>  {
      help::execute();
      process::exit(0);
    },
    Action::List(l) =>  {
      let snippets: Snippet = loader::read_snippets();
      list::execute(l, snippets);
    }
    Action::Run(snippet_key, arga) =>  {
      let snippets: Snippet = loader::read_snippets();
      run::execute(&snippet_key, snippets, &arga);
    }
    _ => {
      println!("Action not implemented: {:?}", a);
      process::exit(3);
    }
  };
}


// "([^"]+)"|\s*([^"\s]+)

