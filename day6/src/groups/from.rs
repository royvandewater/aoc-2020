use super::Group;
use std::collections::HashSet;

impl From<&str> for Group {
  fn from(input: &str) -> Group {
    Group {
      value: calc_value(input),
      value_alt: calc_value_alt(input),
    }
  }
}

fn calc_value(input: &str) -> usize {
  let chars: HashSet<char> = input.replace("\n", "").chars().collect();
  chars.len()
}

fn calc_value_alt(input: &str) -> usize {
  let first_line = input.lines().nth(0).unwrap_or("");
  let mut shared_chars: HashSet<_> = first_line.chars().collect();

  for line in input.lines() {
    let chars: HashSet<char> = line.chars().collect();

    shared_chars = shared_chars.intersection(&chars).cloned().collect();
  }

  shared_chars.len()
}

#[cfg(test)]
mod tests {
  use crate::groups::Group;

  #[test]
  fn value_when_given_single_char() {
    assert_eq!(1, Group::from("a").value);
  }

  #[test]
  fn value_when_given_two_different_chars() {
    assert_eq!(2, Group::from("ab").value);
  }

  #[test]
  fn value_when_given_two_identical_chars() {
    assert_eq!(1, Group::from("aa").value);
  }

  #[test]
  fn value_when_given_two_rows() {
    assert_eq!(2, Group::from("a\nb").value);
  }

  #[test]
  fn value_alt_when_given_single_char() {
    assert_eq!(1, Group::from("a").value_alt);
  }

  #[test]
  fn value_alt_when_given_two_rows_different_char() {
    assert_eq!(0, Group::from("a\nb").value_alt);
  }
}
