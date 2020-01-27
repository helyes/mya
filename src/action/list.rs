use crate::alias::loader;
use crate::alias::model::{Details, Alias};
use crate::util::string_util;
use colored::*;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum ActionListMode {
  Group,
  Short, // this is for bash code completition
  Verbose,
}

fn print_aliases(
  mode: &ActionListMode,
  group: &str,
  available_aliases: BTreeMap<String, Details>,
) {
  match mode {
    ActionListMode::Group => {
        print!("{} ", group);
    }
    ActionListMode::Short => {
      for (key, _value) in &available_aliases {
        print!("{} ", key);
      }
    }
    ActionListMode::Verbose => {
      // println!("\n{}", "Available commands".green().bold());
      println!("\n{}", group.green().bold());
      let mut longest_key_length = 0;
      for (key, _value) in &available_aliases {
        if key.len() > longest_key_length {
          longest_key_length = key.len();
        }
      }
      for (key, value) in &available_aliases {
        let keyp = format!("  {}", key);
        println!(
          "{} {}",
          keyp.yellow().bold(),
          string_util::left_pad(&value.command, longest_key_length - key.len(), ' ').bold()
        );
        println!(
          "{}",
          string_util::left_pad(&value.description, longest_key_length + 3, ' ')
        );
      }
    }
  }
}

pub fn execute(mode: ActionListMode, group: Option<&str>, group_file_path: Option<&String>) -> i32 {
  debug!("Listing aliases");

  if mode == ActionListMode::Verbose {
    println!("\n{}", "Available aliases".green().bold());
  }

  let mut files_to_read: Vec<String> = Vec::new();
  match group_file_path {
    Some(file_path) => {
      debug!("Reading alias from {:?}", file_path);
      let available_aliases: BTreeMap<String, Details>;
      // group is given, we know what file to read
      files_to_read.push(String::from(format!("{}", file_path)));
      let aliases = loader::read_alias_file(&files_to_read[0]);
      match aliases {
        Alias::Commands(value) => {
          // println!("value: {:?}", value);
          available_aliases = value;
        }
      }
      print_aliases(&mode, group.unwrap_or(""), available_aliases)
    }
    None => {
      debug!("No group provided");
      let all_groups: BTreeMap<String, String> = loader::get_group_names();
      for (key, value) in &all_groups {
        let aliases = loader::read_alias_file(&value);
        let available_aliases: BTreeMap<String, Details>;
        match aliases {
          Alias::Commands(value) => {
            // println!("value: {:?}", value);
            available_aliases = value;
          }
        }
        print_aliases(&mode, key, available_aliases)
      }
    }
  }

  return 0;
}
