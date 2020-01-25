pub fn left_pad(s: &str, pad: usize, padchar: char) -> String
{
    let mut ret = String::new();
    for _ in 0..pad {
        ret.push(padchar);
    }
    ret.push_str(s);
    ret
}

pub fn personalize(command: &str, args: &[String]) -> String {
  debug!("Personalizing string: {} with with values{:?}", command, args);
  let mut ret = String::from(command).as_str().to_owned();
  for i in 0..args.len() {
    let placeholder = format!("{{{}}}", i+1);
    debug!("Replacing {} to {}", placeholder,  args[i]);
    ret = ret.replace(&placeholder, &args[i]);
  }

  //expand env variables and ~
  debug!("Expanding string: {}", ret);
  let ret_expanded = shellexpand::full(&ret).unwrap();
  debug!("Personalized string: {}", ret_expanded);
  return String::from(ret_expanded);
}
