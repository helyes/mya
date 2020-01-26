// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
// use colored::*;
use std::env;
use std::process;

#[macro_use] extern crate log;

mod action;
use crate::action::{ list, run };
mod snippet;
use crate::snippet::{ loader, model::Snippet };
mod util;
extern crate clap;

use clap::{ App, AppSettings, Arg };

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
                  Arg::with_name("short")
                      .short("s")
                      .help("short list")
                ),
        )
        .subcommand(
          App::new("run")
              .about("Runs given alias")
              .setting(AppSettings::ArgRequiredElseHelp)
              .arg(
                Arg::with_name("alias")
                     .help("Alias to run")
                     .required(true)
                     .multiple(true))
        )
        .get_matches();

        match matches.subcommand() {
          ("run", Some(add_matches)) => {
            // Reference to run's matches
            println!("Running alias {:?}", add_matches.values_of("alias").unwrap().collect::<Vec<_>>().join(", "));
              let run_params = add_matches
                        .values_of("alias")
                        .unwrap()
                        .collect::<Vec<_>>();

              let snippet_group: Option<&String>;
              let snippet_key : &str;
              let group_names = loader::get_grop_names();
              match run_params.len() {
               0 => {
                // this scenario can't happen due to clap config
                panic!("Run subcommand must be followed by alias name")
               }
               1 => {
                // no group: mya run taskname was run
                snippet_group = None;
                snippet_key = run_params[0];
               },
               _ => {
                 // figure if second parameter is a group
                snippet_group = group_names.get(run_params[0]);
                match snippet_group {
                  Some(_group) => {
                    snippet_key = run_params[1];
                    &args_to_pass.remove(0);
                  },
                  None => snippet_key = run_params[0],
                }
               },
              };
              // let snippet_key = run_params[0];
              let snippets: Snippet = loader::read_snippets(&snippet_group);
              let snippet_details = snippets.get_details_for(&snippet_key);
              match snippet_details {
                Some(details) => {
                  exit_code = run::execute(&snippet_key, details, &args_to_pass);
                }
                None => exit_code = 4
              }

          }
          ("list", Some(list_matches)) => {
              if list_matches.is_present("short") {
                  // "$ mya list -s" was run
                  let snippets: Snippet = loader::read_snippets(&None);
                  exit_code = list::execute(list::ActionListMode::Short, snippets);
              } else {
                  // "$ mya list" was run
                  let snippets: Snippet = loader::read_snippets(&None);
                  exit_code = list::execute(list::ActionListMode::Verbose, snippets);
              }
          }
          ("", None) => {
            println!("No subcommand was used"); // this can't happen as subcommands are mandatory
            exit_code = 7;
          }, // If no subcommand was usd it'll match the tuple ("", None)
          _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
      }

  process::exit(exit_code);
}
// "([^"]+)"|\s*([^"\s]+)
