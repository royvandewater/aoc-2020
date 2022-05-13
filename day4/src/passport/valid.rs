use std::collections::HashMap;
use std::collections::HashSet;

pub fn is_valid(passport: HashMap<String, String>) -> bool {
  let required_keys: HashSet<String> = HashSet::from([
    "ecl".to_string(),
    "pid".to_string(),
    "eyr".to_string(),
    "hcl".to_string(),
    "byr".to_string(),
    "iyr".to_string(),
    "hgt".to_string(),
  ]);

  let keys: HashSet<String> = HashSet::from_iter(passport.keys().map(|k| k.to_string()));
  return keys.is_superset(&required_keys);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn when_invalid_passport() {
    assert_eq!(false, is_valid(HashMap::new()));
  }

  #[test]
  fn when_valid_passport() {
    assert_eq!(
      true,
      is_valid(HashMap::from([
        ("ecl".to_string(), "gry".to_string()),
        ("pid".to_string(), "860033327".to_string()),
        ("eyr".to_string(), "2020".to_string()),
        ("hcl".to_string(), "#fffffd".to_string()),
        ("byr".to_string(), "1937".to_string()),
        ("iyr".to_string(), "2017".to_string()),
        ("cid".to_string(), "147".to_string()),
        ("hgt".to_string(), "183cm".to_string()),
      ]))
    );
  }
}
