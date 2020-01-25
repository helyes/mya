pub fn left_pad(s: &str, pad: usize, padchar: char) -> String
{
    let mut ret = String::new();
    for _ in 0..pad {
        ret.push(padchar);
    }
    ret.push_str(s);
    ret
}