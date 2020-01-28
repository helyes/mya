use crate::alias::loader;
use crate::action::list;
use colored::*;

pub fn handle(list_matches: &clap::ArgMatches<'_>) -> i32 {
  let gn = loader::get_group_names();
  let group_names;
  match gn {
    Ok(groups) => group_names = groups,
    Err(err) => {
      println!("{}", err);
      return 27;
    }
  }

  let run_params_options = list_matches.values_of("alias");
  let run_params :Vec<_>;

  match run_params_options {
    Some(options) => {
      debug!("Parameters: {:?}", options);
      run_params = options.collect::<Vec<_>>();
    }
    None =>  {
      debug!("No parameters given");
      // TODO: refactor
      run_params = Vec::new();
    }
  }

  // alias path
  let group_file_path: Option<&String>;

  // alias group coming from paraemters,
  let group: Option<&str>;
  match run_params.len() {
    0 => {
      // no group given
      // "$ mya list" was run
      group_file_path = None;
      group = None;
    }
    _ => {
      // group set - only care about first one
      // "$ mya list shell" was run
      group = Some(run_params[0]);
      if !group_names.contains_key(run_params[0]) {
        println!("\nGroup {} does not exist", run_params[0].red());
        return 7;
      } 
      group_file_path = group_names.get(run_params[0]);
    }
  };

  debug!("Working on group: {:?}", group_file_path);

   if list_matches.is_present("groupsonly") {
      // "$ mya list -g" was run
      return list::execute(list::ActionListMode::Group, group, group_file_path);
   } else if list_matches.is_present("short") {
      // "$ mya list -s" was run
      return list::execute(list::ActionListMode::Short, group, group_file_path);
    } else {
    // "$ mya list" was run
    return list::execute(list::ActionListMode::Verbose, group, group_file_path);
  }
}
