use std::env;
use std::fs;

use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// struct Application {
//     application: Environment,
// }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Command {
    command: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum Environment {
    Hash(BTreeMap<String, Command>),
}

fn main() {

  let content_hash = r#"
  {
        "ONE_ENV": 
          {command : "fake" }
  }
  "#;
  let application_data: Environment = serde_yaml::from_str(content_hash).unwrap();
  println!("{:?}", application_data);
  

  match serde_yaml::to_string(&application_data) {
      Ok(s1) => println!("{:?}", s1), 
      Err(_) => { println!("Erro") },
  };
  

  let file_to_read = get_snipet_file_path();
  println!("Reading file: {}", file_to_read);

  let contents = fs::read_to_string(file_to_read)
    .expect("Something went wrong reading the file");

  println!("With text:\n{}", contents);

}

fn get_snipet_file_path() -> String {
  let file_path = String::from("snippets/shell.yaml"); // some_string comes into scope
  file_path                              
}

// fn parse_config(args: &[String]) -> (&str, &str) {
//   let par1 = &args[1];
//   let par2 = &args[2];

//   (par1, par2)
// }