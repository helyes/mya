// https://doc.rust-lang.org/rust-by-example/custom_types/enum/testcase_linked_list.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
use std::process;

mod action;
mod action_handler;
mod alias;
mod util;

#[macro_use]
extern crate log;
extern crate clap;
use clap::{App, AppSettings, Arg};

fn main() {
  env_logger::init();
  let exit_code: i32;
  // https://github.com/brocode/fw/blob/c803cd20dd6f5f4f07fb4c061f7ac9e8240ea29b/src/app/mod.rs
  let matches = App::new("mya")
    .about("My better aliases")
    .version("1.0")
    .global_setting(AppSettings::ColoredHelp)
    .setting(AppSettings::SubcommandRequired)
    .author("Andras <helyesa@gmail.com>")
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
    ("run", Some(run_matches)) => {
      exit_code = action_handler::run::handle(run_matches);
    }
    ("list", Some(list_matches)) => {
      exit_code = action_handler::list::handle(list_matches);
    }
    ("", None) => {
      println!("No subcommand was used"); // this can't happen as subcommands are mandatory
      exit_code = 7;
    } // If no subcommand was used it'll match the tuple ("", None)
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
  }

  process::exit(exit_code);
}
