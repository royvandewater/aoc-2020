mod passport;
pub use passport::Passport;

pub fn collect_passport_strings(s: &str) -> Vec<String> {
  match s {
    "" => vec![],
    _ => s
      .split("\n\n")
      .map(|i| i.replace("\n", " ").trim().to_string())
      .collect(),
  }
}

pub fn valid_passport_string(s: &str) -> bool {
  Passport::from(s).is_valid()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn collect_passport_strings_when_empty() {
    let expected: Vec<String> = vec![];

    assert_eq!(expected, collect_passport_strings(""))
  }

  #[test]
  fn collect_passport_strings_when_two_single_line_passports() {
    let expected: Vec<String> = vec!["ecl:gry".into(), "ecl:ylw".into()];

    assert_eq!(
      expected,
      collect_passport_strings(
        "
ecl:gry

ecl:ylw
    "
      )
    )
  }

  #[test]
  fn collect_passport_strings_when_two_multi_line_passports() {
    let expected: Vec<String> = vec!["ecl:gry pid:123".into(), "ecl:ylw pid:456".into()];

    assert_eq!(
      expected,
      collect_passport_strings(
        "
ecl:gry
pid:123

ecl:ylw
pid:456
    "
      )
    )
  }

  #[test]
  fn when_invalid_passport() {
    assert_eq!(false, valid_passport_string(""));
  }

  #[test]
  fn when_valid_passport() {
    assert_eq!(
      true,
      valid_passport_string(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
      )
    );
  }
}
