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