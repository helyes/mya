use regex::Regex;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

// https://serde.rs/derive.html
// https://github.com/dtolnay/serde-yaml
// https://stackoverflow.com/questions/55245914/how-to-use-serde-to-parse-a-yaml-file-with-multiple-different-types
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Details {
    pub command: String,
    pub description: String,
    pub directory: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Snippet {
    Commands(BTreeMap<String, Details>),
}

impl Details {
  pub fn get_command_without_parameters (&self) -> String {
    debug!("Getting command without parameters...");
    if !self.command.trim().contains(" ") {
      return String::from(&self.command);
    }

    let re = Regex::new(r#"^"(.*?)".*"#).unwrap();
    let mut captures = re.captures(&self.command);
    let mut ret;
    match captures {
      None => ret = "",
      _ => ret = captures.unwrap().get(1).map_or("", |m| m.as_str()),
    }
    if ret.is_empty() {
      let re = Regex::new(r"^(.*?)\s").unwrap();
      captures = re.captures(&self.command);
      ret = captures.unwrap().get(1).map_or("", |m| m.as_str());
    }
    debug!("  Command to execute: {}", &ret);
    return String::from(ret);
  }

  pub fn get_command_arguments (&self) -> Vec<String> {
    // ideal regex, not supported: "(.+?)"|([-\w]+(?=\s|$))
    let re = Regex::new(r#"("[^"]+")|\S+"#).unwrap();
    let mut vec = Vec::new();
    debug!("Getting arguments for command...");
    let mut i: i16 = 0;
    for cap in re.captures_iter(&self.command) {
      // skip firs argument. That's the script name
      if i!= 0 {
        // debug!("  Found argument: {}", &cap[0]);
        vec.push(String::from(&cap[0]));
      }
      i = i+1;
    }
    debug!("  Arguments: {:?}", vec);
    return vec;
  }
}
