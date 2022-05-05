pub fn parse_range(range: &str) -> Option<(usize, usize)> {
  let mut parts = range.split("-");
  let min = parts.next()?.parse::<usize>().ok()?;
  let max = parts.next()?.parse::<usize>().ok()?;

  Some((min, max))
}

pub fn parse_rule(rule: &str) -> Option<(usize, usize, char)> {
  let mut parts = rule.split(" ");
  let range = parts.next()?;
  let character = parts.next()?.chars().next()?;

  let (min, max) = parse_range(range)?;

  Some((min, max, character))
}

pub fn parse_line(line: &str) -> Option<(usize, usize, char, String)> {
  let mut parts = line.split(": ");
  let rule = parts.next()?;
  let password = parts.next()?;

  let (min, max, character) = parse_rule(rule)?;

  Some((min, max, character, password.to_string()))
}

pub fn is_valid_password(line: &str) -> bool {
  let (min, max, character, password) =
    parse_line(line).expect(format!("couldn't parse line: '{}'", line).as_str());

  let character_count = password.chars().filter(|c| *c == character).count();

  min <= character_count && character_count <= max
}

pub fn is_valid_type_2_password(line: &str) -> bool {
  let (min, max, character, password) =
    parse_line(line).expect(format!("couldn't parse line: '{}'", line).as_str());

  let a = password
    .chars()
    .nth(min - 1)
    .expect("min index not found in password");
  let b = password
    .chars()
    .nth(max - 1)
    .expect("max index not found in password");

  let character_count = vec![a, b].iter().filter(|c| **c == character).count();

  character_count == 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn when_valid_password_then_true() {
    assert_eq!(true, is_valid_password("1-3 a: abcde"));
  }

  #[test]
  fn when_not_valid_password_then_false() {
    assert_eq!(false, is_valid_password("1-3 b: cdefg"));
  }

  #[test]
  fn when_valid_other_password_then_true() {
    assert_eq!(true, is_valid_password("2-9 c: ccccccccc"));
  }

  #[test]
  fn when_valid_type_2_password_then_true() {
    assert_eq!(true, is_valid_type_2_password("1-3 a: abcde"));
  }

  #[test]
  fn when_not_valid_type_2_password_then_false() {
    assert_eq!(false, is_valid_type_2_password("1-3 b: cdefg"));
  }

  #[test]
  fn when_not_valid_different_type_2_password_then_false() {
    assert_eq!(false, is_valid_type_2_password("2-9 c: ccccccccc"));
  }
}
