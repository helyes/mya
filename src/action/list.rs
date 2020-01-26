use std::collections::BTreeMap;
use colored::*;
use crate::util::string_util;
use crate::snippet::model::{ Snippet, Details};
use crate::snippet::{loader};

#[derive(Debug)]
pub enum ActionListMode {
  Short, // this is for bash code completition
  Verbose
}

pub fn execute(mode: ActionListMode, group_file_path: Option<&String>) -> i32 {
  debug!("Listing snippets");
  let file_to_read: String;
  match group_file_path {
    Some(group) => {
      debug!("Reading snippet from {:?}", group);
      // group is given, we know what file to read
      file_to_read = String::from(format!("{}", group));
      let snippets = loader::read_snippet_file(&file_to_read);
      //return Some(read_snippet_file(&file_to_read));
      let available_snippets: BTreeMap<String, Details>;
      match snippets {
        Snippet::Commands(value) => {
          // println!("value: {:?}", value);
          available_snippets = value;
        }
      }
      match mode {
        ActionListMode::Short => {
          for (key, _value) in &available_snippets {
            print!("{} ", key);
          };
        },
        ActionListMode::Verbose => {
          println!("\n{}", "Available commands".green().bold());
          let mut longest_key_length = 0;
          for (key, _value) in &available_snippets {
            if key.len() > longest_key_length {
              longest_key_length = key.len();
            }
          }
          for (key, value) in &available_snippets {
            let keyp = format!("  {}", key);
            println!("{} {}", keyp.yellow().bold(), string_util::left_pad(&value.command, longest_key_length - key.len(), ' ').bold());
            println!("{}", string_util::left_pad(&value.description, longest_key_length + 3, ' '));
          }
        }
      }
    }
    None => {
      debug!("No group provided");
      // let group_names = get_group_names();
      // for (_key, value) in group_names {
      //   let snippet = read_snippet_file(&value);
      //   if snippet.is_key_exists(snippet_key) {
      //     return Some(snippet);
      //   }
      // }
      // return None;
    }
  }
  return 0;
}

pub fn execute_o(mode: ActionListMode, snippets: Snippet) -> i32 {
  debug!("Listing snippets ss");

  debug!("Snippets: {:?}", snippets);
  let available_snippets: BTreeMap<String, Details>;
  match snippets {
    Snippet::Commands(value) => {
      // println!("value: {:?}", value);
      available_snippets = value;
    }
  }
  //println!("Available Snippets: {:?}", available_snippets);

  match mode {
    ActionListMode::Short => {
      for (key, _value) in &available_snippets {
        print!("{} ", key);
      };
    },
    ActionListMode::Verbose => {
      println!("\n{}", "Available commands".green().bold());
      let mut longest_key_length = 0;
      for (key, _value) in &available_snippets {
        if key.len() > longest_key_length {
          longest_key_length = key.len();
        }
      }
      for (key, value) in &available_snippets {
        let keyp = format!("  {}", key);
        println!("{} {}", keyp.yellow().bold(), string_util::left_pad(&value.command, longest_key_length - key.len(), ' ').bold());
        println!("{}", string_util::left_pad(&value.description, longest_key_length + 3, ' '));
      }
    }
  }
  return 0;
}