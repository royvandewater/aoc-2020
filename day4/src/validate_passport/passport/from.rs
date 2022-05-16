pub use super::Passport;
use std::convert::From;

impl From<&str> for Passport {
  fn from(s: &str) -> Self {
    let mut passport = Passport::new();

    for pair in s.split_whitespace().map(|i| parse_item(i)) {
      let key = pair.0.as_str();
      let value = pair.1.as_str();

      match key {
        "ecl" => passport.set_ecl(value).unwrap_or(()),
        "pid" => passport.set_pid(value).unwrap_or(()),
        "eyr" => passport.set_eyr(value).unwrap_or(()),
        "hcl" => passport.set_hcl(value).unwrap_or(()),
        "byr" => passport.set_byr(value).unwrap_or(()),
        "iyr" => passport.set_iyr(value).unwrap_or(()),
        "hgt" => passport.set_hgt(value).unwrap_or(()),
        _ => {}
      }
    }

    return passport;
  }
}

fn parse_item(s: &str) -> (String, String) {
  let parts: Vec<String> = s.split(":").map(|p| p.to_string()).collect();
  return (parts[0].to_string(), parts[1].to_string());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn when_given_empty_input_returns_empty() {
    assert_eq!(Passport::new(), Passport::from(""));
  }

  #[test]
  fn when_single_kv_pair() -> Result<(), String> {
    let mut expected = Passport::new();
    expected.set_ecl("gry")?;

    assert_eq!(expected, Passport::from("ecl:gry"));
    Ok(())
  }

  #[test]
  fn when_two_kv_pairs() -> Result<(), String> {
    let mut expected = Passport::new();
    expected.set_ecl("gry")?;
    expected.set_pid("860033327")?;

    assert_eq!(expected, Passport::from("ecl:gry pid:860033327"));
    Ok(())
  }

  #[test]
  fn when_full_passport() -> Result<(), String> {
    let mut expected = Passport::new();
    expected.set_ecl("gry")?;
    expected.set_pid("860033327")?;
    expected.set_eyr("2020")?;
    expected.set_hcl("#fffffd")?;
    expected.set_byr("1937")?;
    expected.set_iyr("2017")?;
    expected.set_hgt("183cm")?;

    assert_eq!(
      expected,
      Passport::from(
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm"
      )
    );
    Ok(())
  }
}
