use regex::Regex;

pub fn left_pad(s: &str, pad: usize, padchar: char) -> String {
  let mut ret = String::new();
  for _ in 0..pad {
    ret.push(padchar);
  }
  ret.push_str(s);
  ret
}

pub fn personalize(command: &str, args: &[String]) -> Result<String, &'static str> {
  debug!(
    "Personalizing string: {} with with values{:?}",
    command, args
  );
  let mut ret = String::from(command).as_str().to_owned();
  for i in 0..args.len() {
    // placeholder = {{1}} etc...
    let placeholder = format!("{{{}}}", i + 1);
    debug!("Replacing {} to {}", placeholder, args[i]);
    ret = ret.replace(&placeholder, &args[i]);
  }

  //expand env variables and ~
  debug!("Expanding string: {}", ret);
  let ret = shellexpand::full(&ret).unwrap();
  debug!("Personalized string: {}", ret);

  let regex = Regex::new(r"(?m)\{\d+\}").unwrap();
  if regex.is_match(&ret) {
    println!("\nAssembled command: {}", ret);
    return Err("Not enough parameters for command");
  }
  return Ok(String::from(ret));
}

#[cfg(test)]
mod tests_left_pad {
  use super::*;
  #[test]
  fn pad_2_space() {
    assert_eq!("  message ", left_pad("message ", 2, ' '));
  }

  #[test]
  fn pad_3_x() {
    assert_eq!("xxxmessage ", left_pad("message ", 3, 'x'));
  }

  #[test]
  fn pad_3_empty_str() {
    assert_eq!("   ", left_pad("", 3, ' '));
  }

  #[test]
  fn pad_0() {
    assert_eq!("message", left_pad("message", 0, ' '));
  }
}

#[cfg(test)]
mod tests_personalize {
  use super::*;
  #[test]
  fn with_env() {
    let raw = "ls -l ${HOME}/work/{1}/db ${HOME}/{2}";
    let personalized = personalize(raw, &[String::from("passed1"), String::from("passed2")]);
    let expected = shellexpand::full("ls -l ${HOME}/work/passed1/db ${HOME}/passed2").unwrap();
    assert_eq!(personalized.unwrap(), String::from(expected));
  }

  #[test]
  fn with_tilda() {
    let raw = "ls -l ~/work/sc/{1}/db ~/{2}";
    let personalized = personalize(raw, &[String::from("passed1"), String::from("passed2")]);
    let expected = shellexpand::full("ls -l ~/work/sc/passed1/db ~/passed2").unwrap();
    assert_eq!(personalized.unwrap(), String::from(expected));
  }

  #[test]
  fn more_placeholders_than_params() {
    let raw = "ls -l {1}/db ~/{2} ~/{3}";
    let personalized = personalize(raw, &[String::from("passed1"), String::from("passed2")]);
    assert_eq!(personalized, Err("Not enough parameters for command"));
  }

  #[test]
  fn handles_gap_in_placeholders() {
    let raw = "ls -l {1}/db ~/{3}";
    let personalized = personalize(
      raw,
      &[
        String::from("passed1"),
        String::from("passed2"),
        String::from("passed3"),
      ],
    );
    assert_eq!(
      personalized.unwrap(),
      String::from("ls -l passed1/db ~/passed3")
    );
  }
}
