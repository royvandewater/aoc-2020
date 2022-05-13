use std::collections::HashMap;

fn parse_item(s: &str) -> (String, String) {
  let parts: Vec<String> = s.split(":").map(|p| p.to_string()).collect();
  return (parts[0].to_string(), parts[1].to_string());
}

pub fn parse(s: &str) -> HashMap<String, String> {
  if !s.contains(":") {
    return HashMap::new();
  }

  return HashMap::from_iter(s.split(" ").map(|i| parse_item(i)));
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn when_given_empty_input_returns_empty() {
    assert_eq!(HashMap::from([]), parse(""));
  }

  #[test]
  fn when_single_kv_pair() {
    assert_eq!(
      HashMap::from([("ecl".to_string(), "gry".to_string())]),
      parse("ecl:gry")
    );
  }

  #[test]
  fn when_two_kv_pairs() {
    assert_eq!(
      HashMap::from([
        ("ecl".to_string(), "gry".to_string()),
        ("pid".to_string(), "860033327".to_string()),
      ]),
      parse("ecl:gry pid:860033327")
    );
  }
}
