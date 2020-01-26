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

pub fn get_group_names() -> BTreeMap<String, String> {
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

pub fn read_snippet_file(file_to_read: &str) -> Snippet {
  let contents = fs::read_to_string(file_to_read)
      .expect(format!("Something went wrong reading file: {}", file_to_read).as_str());
  let snippets: Snippet = serde_yaml::from_str(&contents).unwrap();
  return snippets;
}

pub fn get_snippet_for_key(group: &Option<&String>, snippet_key: &str) -> Option<Snippet> {
  // let snippet_files_folder = get_snipet_files_folder();
  let file_to_read: String;
  match group {
    Some(group) => {
      debug!("Reading snippet for group: {:?}", group);
      // group is given, we know what file to read
      file_to_read = String::from(format!("{}", group));
      let snippet = read_snippet_file(&file_to_read);
      //return Some(read_snippet_file(&file_to_read));
      if snippet.is_key_exists(snippet_key) {
        return Some(snippet);
      } else {
        return None;
      }
    }
    None => {
      debug!("No group provided, searching {} in all snippet files", snippet_key);
      let group_names = get_group_names();
      for (_key, value) in group_names {
        let snippet = read_snippet_file(&value);
        if snippet.is_key_exists(snippet_key) {
          return Some(snippet);
        }
      }
      return None;
    }
  }
}