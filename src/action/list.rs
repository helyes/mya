use std::collections::BTreeMap;
use colored::*;
use crate::util::string_util;
use crate::snippet::model::{ Snippet, Details};

#[derive(Debug)]
pub enum ActionListMode {
  Short, // this is for bash code completition
  Verbose
}

pub fn execute(mode: ActionListMode, snippets: Snippet) {
  debug!("Listing snippets");

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
      }
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