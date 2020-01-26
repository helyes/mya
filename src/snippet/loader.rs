use std::env;
use std::fs;

use crate::snippet::model::Snippet;

fn get_snipet_files_folder() -> String {
  match env::var_os("MYA_ALIASES_HOME") {
    Some(val) => {
      let env_path = val.to_str().unwrap();
      let expanded = shellexpand::full(env_path).unwrap();
      return String::from(expanded);
    },
    None => String::from(env::var_os("HOME").unwrap().to_str().unwrap())
  }
}

pub fn get_grop_names() -> Option<Vec<(String, String)>> {
  let snippet_files_folder = get_snipet_files_folder();
  let entries = fs::read_dir(&snippet_files_folder);
    match entries {
      Ok(entries) => {
        let mut ret = Vec::<(String, String)>::new();
        for r in entries {
          let path = r.unwrap().path();
          let category = String::from(path.file_stem().unwrap().to_str().unwrap());
          ret.push((category, path.into_os_string().into_string().unwrap()));
        }
        return Some(ret);
      }
      Err(message) => {
        println!("Error loading files from folder:{}, Error: {:?}", &snippet_files_folder, message);
        return None;
      }
    }
}

pub fn read_snippets(group: Option<&str>) -> Snippet {

  let snippet_root = "snippets";
  let file_to_read: String; // = get_snipet_file_path();
  match group {
    Some(group) => {
      file_to_read = String::from(format!("{}/{}", snippet_root, group))
    }
    None => file_to_read = String::from("snippets/shell.yaml")
  }

  debug!("Reading file: {}", file_to_read);

  let contents = fs::read_to_string(file_to_read)
    .expect("Something went wrong reading the file");
  let snippets: Snippet = serde_yaml::from_str(&contents).unwrap();
  return snippets;
}