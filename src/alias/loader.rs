use crate::alias::model::Alias;
use colored::*;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::Path;

fn get_alias_files_folder() -> String {
  match env::var_os("MYA_ALIASES_HOME") {
    Some(val) => {
      let env_path = val.to_str().unwrap();
      let expanded = shellexpand::full(env_path).unwrap();
      return String::from(expanded);
    }
    None => {
      let home = String::from(env::var_os("HOME").unwrap().to_str().unwrap());
      let ret: String;
      ret = String::from(Path::new(&home).join(".mya").to_str().unwrap());
      return ret;
    }
  }
}

pub fn get_group_names() -> Result<BTreeMap<String, String>, &'static str> {
  let alias_files_folder = get_alias_files_folder();
  debug!("Getting group names from folder: {}", alias_files_folder);
  let mut ret = BTreeMap::new();
  let entries = fs::read_dir(&alias_files_folder);
  match entries {
    Ok(entries) => {
      for r in entries {
        let path = r.unwrap().path();
        let category = String::from(path.file_stem().unwrap().to_str().unwrap());
        ret.insert(category, path.into_os_string().into_string().unwrap());
      }
      debug!("Group names: {:?}", ret);
      return Ok(ret);
    }
    Err(err) => {
      println!(
        "{} {}, Error: {:?}",
        "Error loading files from folder:".red(),
        &alias_files_folder.red(),
        err
      );
      return Err("Could not load aliases");
    }
  }
}

pub fn read_alias_file(file_to_read: &str) -> Alias {
  let contents = fs::read_to_string(file_to_read)
    .expect(format!("Something went wrong reading file: {}", file_to_read).as_str());
  let aliases: Alias = serde_yaml::from_str(&contents).unwrap();
  return aliases;
}

pub fn get_alias_for_key(group: &Option<&String>, alias_key: &str) -> Option<Alias> {
  let file_to_read: String;
  match group {
    Some(group) => {
      debug!("Reading alias for group: {:?}", group);
      // group is given, we know what file to read
      file_to_read = String::from(format!("{}", group));
      let alias = read_alias_file(&file_to_read);
      //return Some(read_alias_file(&file_to_read));
      if alias.is_key_exists(alias_key) {
        return Some(alias);
      } else {
        return None;
      }
    }
    None => {
      debug!(
        "No group provided, searching {} in all alias files",
        alias_key
      );

      let gn = get_group_names();
      // let group_names;
      match gn {
        Ok(group_names) => {
          // group_names = groups;
          for (_key, value) in group_names {
            let alias = read_alias_file(&value);
            if alias.is_key_exists(alias_key) {
              return Some(alias);
            }
          }
          return None;
        }
        Err(err) => {
          println!("{}", err);
          return None;
        }
      }
    }
  }
}
