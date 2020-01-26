use std::env;
use std::fs;
use std::collections::BTreeMap;
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

pub fn get_grop_names() -> BTreeMap<String, String> {
  let snippet_files_folder = get_snipet_files_folder();
  debug!("Getting group names from folder: {}", snippet_files_folder);
  let mut ret = BTreeMap::new();
  let entries = fs::read_dir(&snippet_files_folder);
    match entries {
      Ok(entries) => {
        for r in entries {
          let path = r.unwrap().path();
          let category = String::from(path.file_stem().unwrap().to_str().unwrap());
          ret.insert(category, path.into_os_string().into_string().unwrap());
        }
        debug!("Group names: {:?}", ret);
        return ret;
      }
      Err(message) => {
        println!("Error loading files from folder:{}, Error: {:?}", &snippet_files_folder, message);
        return ret;
      }
    }
}

pub fn read_snippets(group: &Option<&String>) -> Snippet {
  debug!("Reading snippet for group: {:?}", group);

  // let snippet_files_folder = get_snipet_files_folder();
  let file_to_read: String; // = get_snipet_file_path();
  match group {
    Some(group) => {
      file_to_read = String::from(format!("{}", group))
    }
    None => file_to_read = String::from("snippets/shell.yaml")
  }

  debug!("Reading file: {}", file_to_read);

  let contents = fs::read_to_string(file_to_read)
    .expect("Something went wrong reading the file");
  let snippets: Snippet = serde_yaml::from_str(&contents).unwrap();
  return snippets;
}