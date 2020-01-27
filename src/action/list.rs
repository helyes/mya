use crate::snippet::loader;
use crate::snippet::model::{Details, Snippet};
use crate::util::string_util;
use colored::*;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum ActionListMode {
  Short, // this is for bash code completition
  Verbose,
}

fn print_snippets(
  mode: &ActionListMode,
  group: &str,
  available_snippets: BTreeMap<String, Details>,
) {
  match mode {
    ActionListMode::Short => {
      for (key, _value) in &available_snippets {
        print!("{} ", key);
      }
    }
    ActionListMode::Verbose => {
      // println!("\n{}", "Available commands".green().bold());
      println!("\n{}", group.green().bold());
      let mut longest_key_length = 0;
      for (key, _value) in &available_snippets {
        if key.len() > longest_key_length {
          longest_key_length = key.len();
        }
      }
      for (key, value) in &available_snippets {
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

pub fn execute(mode: ActionListMode, snippet_requested: Option<&str>, group_file_path: Option<&String>) -> i32 {
  debug!("Listing snippets");

  if mode != ActionListMode::Short {
    println!("\n{}", "Available aliases".green().bold());
  }
  let mut files_to_read: Vec<String> = Vec::new();
  match group_file_path {
    Some(file_path) => {
      debug!("Reading snippet from {:?}", file_path);
      let available_snippets: BTreeMap<String, Details>;
      // group is given, we know what file to read
      files_to_read.push(String::from(format!("{}", file_path)));
      let snippets = loader::read_snippet_file(&files_to_read[0]);
      match snippets {
        Snippet::Commands(value) => {
          // println!("value: {:?}", value);
          available_snippets = value;
        }
      }
      print_snippets(&mode, snippet_requested.unwrap_or(""), available_snippets)
    }
    None => {
      debug!("No group provided");
      let all_groups: BTreeMap<String, String> = loader::get_group_names();
      for (key, value) in &all_groups {
        let snippets = loader::read_snippet_file(&value);
        let available_snippets: BTreeMap<String, Details>;
        match snippets {
          Snippet::Commands(value) => {
            // println!("value: {:?}", value);
            available_snippets = value;
          }
        }
        print_snippets(&mode, key, available_snippets)
      }
    }
  }

  return 0;
}
