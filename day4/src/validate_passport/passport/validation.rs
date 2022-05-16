pub use super::Passport;
use std::collections::HashSet;

lazy_static! {
  static ref VALID_ECLS: HashSet<&'static str> =
    HashSet::from(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
}

impl Passport {
  pub fn is_valid(&self) -> bool {
    self.ecl.is_some()
      && self.pid.is_some()
      && self.eyr.is_some()
      && self.hcl.is_some()
      && self.byr.is_some()
      && self.iyr.is_some()
      && self.hgt.is_some()
  }

  pub fn set_ecl(&mut self, ecl: &str) -> Result<(), String> {
    match ecl {
      _ if !VALID_ECLS.contains(ecl) => {
        Err(format!("ecl value not one of allowed values: {}", ecl))
      }
      _ => {
        self.ecl = Some(ecl.into());
        Ok(())
      }
    }
  }

  pub fn set_pid(&mut self, pid: &str) -> Result<(), String> {
    match pid {
      _ if pid.len() != 9 => Err(format!("Length was not 9, was: {}", pid.len())),
      _ if pid.chars().any(|c| !is_digit(c)) => Err(format!("Value contained non digit: {}", pid)),
      value => {
        self.pid = Some(value.into());
        Ok(())
      }
    }
  }

  pub fn set_eyr(&mut self, eyr: &str) -> Result<(), String> {
    match u16::from_str_radix(eyr, 10) {
      Err(_) => Err(format!("Could not parse as u16: {}", eyr)),
      Ok(value) => match value {
        _ if value < 2020 => Err(format!("Eyr was less than 2020: {}", value)),
        _ if 2030 < value => Err(format!("Eyr was greater than 2030: {}", value)),
        value => {
          self.eyr = Some(value);
          Ok(())
        }
      },
    }
  }

  pub fn set_hcl(&mut self, hcl: &str) -> Result<(), String> {
    match hcl {
      _ if !hcl.starts_with("#") => Err(format!("hcl didn't start with a '#': {}", hcl)),
      _ if hcl.len() != 7 => Err(format!("hcl wasn't 7 chars long: {}", hcl)),
      _ if hcl[1..].chars().any(|c| !is_hex_digit(c)) => Err(format!(
        "at least one digit after '#' was not a hex value: {}",
        hcl
      )),
      val => {
        self.hcl = Some(val.into());
        Ok(())
      }
    }
  }

  pub fn set_byr(&mut self, byr: &str) -> Result<(), String> {
    match u16::from_str_radix(byr, 10) {
      Err(_) => Err(format!("Could not parse byr as u16: {}", byr)),
      Ok(value) => match value {
        _ if value < 1920 => Err(format!("byr was less than 1920: {}", byr)),
        _ if 2002 < value => Err(format!("byr was more than 2002: {}", byr)),
        _ => {
          self.byr = Some(value);
          Ok(())
        }
      },
    }
  }

  pub fn set_iyr(&mut self, iyr: &str) -> Result<(), String> {
    match u16::from_str_radix(iyr, 10) {
      Err(_) => Err(format!("Could not parse iyr as u16: {}", iyr)),
      Ok(value) => match value {
        _ if value < 2010 => Err(format!("iyr was less than 2010: {}", iyr)),
        _ if 2020 < value => Err(format!("byr was more than 2020: {}", iyr)),
        _ => {
          self.iyr = Some(value);
          Ok(())
        }
      },
    }
  }

  fn parse_hgt_cm(&mut self, hgt: &str) -> Result<String, String> {
    let num_str = hgt.strip_suffix("cm").unwrap_or(hgt);

    match usize::from_str_radix(num_str, 10) {
      Err(_) => Err(format!("Could not parse xxxcm of hgt as number: {}", hgt)),
      Ok(num) => match num {
        _ if num < 150 => Err(format!("hgt in cm below 150cm: {}", hgt)),
        _ if 193 < num => Err(format!("hgt in cm above 193cm: {}", hgt)),
        _ => Ok(hgt.into()),
      },
    }
  }

  fn parse_hgt_in(&mut self, hgt: &str) -> Result<String, String> {
    let num_str = hgt.strip_suffix("in").unwrap_or(hgt);

    match usize::from_str_radix(num_str, 10) {
      Err(_) => Err(format!("Could not parse xxxin of hgt as number: {}", hgt)),
      Ok(num) => match num {
        _ if num < 59 => Err(format!("hgt in in below 59in: {}", hgt)),
        _ if 76 < num => Err(format!("hgt in in above 76in: {}", hgt)),
        _ => Ok(hgt.into()),
      },
    }
  }

  pub fn set_hgt(&mut self, hgt: &str) -> Result<(), String> {
    match hgt {
      _ if hgt.ends_with("cm") => {
        self.hgt = Some(self.parse_hgt_cm(hgt)?);
        Ok(())
      }
      _ if hgt.ends_with("in") => {
        self.hgt = Some(self.parse_hgt_in(hgt)?);
        Ok(())
      }
      _ => Err(format!("hgt had invalid suffix: {}", hgt)),
    }
  }
}

fn is_digit(c: char) -> bool {
  match c {
    '0'..='9' => true,
    _ => false,
  }
}

fn is_hex_digit(c: char) -> bool {
  match c {
    '0'..='9' => true,
    'a'..='f' => true,
    _ => false,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn when_invalid_passport() {
    assert_eq!(false, Passport::new().is_valid());
  }

  #[test]
  fn when_valid_passport() -> Result<(), String> {
    let mut passport = Passport::new();
    passport.set_ecl("gry")?;
    passport.set_pid("860033327")?;
    passport.set_eyr("2020")?;
    passport.set_hcl("#fffffd")?;
    passport.set_byr("1937")?;
    passport.set_iyr("2017")?;
    passport.set_hgt("183cm")?;

    // ("cid".to_string(), "147".to_string()),
    // ("hgt".to_string(), "183cm".to_string()),

    assert!(passport.is_valid());
    Ok(())
  }

  #[test]
  fn when_setting_ecl_gry() -> Result<(), String> {
    let mut passport = Passport::new();
    passport.set_ecl("gry")?;
    assert_eq!(Some("gry".into()), passport.ecl);
    Ok(())
  }

  #[test]
  fn when_setting_pid_860033327() -> Result<(), String> {
    let mut passport = Passport::new();
    passport.set_pid("860033327")?;

    assert_eq!(Some("860033327".into()), passport.pid);
    Ok(())
  }

  #[test]
  fn when_setting_hcl_valid() -> Result<(), String> {
    let mut passport = Passport::new();
    passport.set_hcl("#fffffd")?;

    assert_eq!(Some("#fffffd".into()), passport.hcl);
    Ok(())
  }

  #[test]
  fn when_setting_hcl_no_hash() {
    let mut passport = Passport::new();
    assert!(passport.set_hcl("0fffffd").is_err());
    assert_eq!(None, passport.hcl);
  }

  #[test]
  fn when_setting_hcl_wrong_length() {
    let mut passport = Passport::new();
    assert!(passport.set_hcl("#ff").is_err());
    assert_eq!(None, passport.hcl);
  }

  #[test]
  fn when_setting_hcl_non_hex() {
    let mut passport = Passport::new();
    assert!(passport.set_hcl("#fffffx").is_err());
    assert_eq!(None, passport.hcl);
  }

  #[test]
  fn when_setting_byr_1937() -> Result<(), String> {
    let mut passport = Passport::new();
    passport.set_byr("1937")?;

    assert_eq!(Some(1937), passport.byr);
    Ok(())
  }

  #[test]
  fn when_setting_byr_non_numeric() {
    let mut passport = Passport::new();
    assert!(passport.set_byr("foo").is_err());
    assert_eq!(None, passport.byr);
  }

  #[test]
  fn when_setting_byr_too_low() {
    let mut passport = Passport::new();
    assert!(passport.set_byr("1900").is_err());
    assert_eq!(None, passport.byr);
  }

  #[test]
  fn when_setting_byr_too_high() {
    let mut passport = Passport::new();
    assert!(passport.set_byr("2003").is_err());
    assert_eq!(None, passport.byr);
  }

  #[test]
  fn when_setting_iyr_2017() -> Result<(), String> {
    let mut passport = Passport::new();
    passport.set_iyr("2017")?;

    assert_eq!(Some(2017), passport.iyr);
    Ok(())
  }

  #[test]
  fn when_setting_iyr_non_numeric() {
    let mut passport = Passport::new();
    assert!(passport.set_iyr("foo").is_err());
    assert_eq!(None, passport.iyr);
  }

  #[test]
  fn when_setting_iyr_too_low() {
    let mut passport = Passport::new();
    assert!(passport.set_iyr("1900").is_err());
    assert_eq!(None, passport.iyr);
  }

  #[test]
  fn when_setting_iyr_too_high() {
    let mut passport = Passport::new();
    assert!(passport.set_iyr("3000").is_err());
    assert_eq!(None, passport.iyr);
  }

  #[test]
  fn when_setting_hgt_183cm() -> Result<(), String> {
    let mut passport = Passport::new();
    passport.set_hgt("183cm")?;

    assert_eq!(Some("183cm".into()), passport.hgt);
    Ok(())
  }

  #[test]
  fn when_setting_hgt_not_valid_unit() {
    let mut passport = Passport::new();
    assert!(passport.set_hgt("183lb").is_err());
    assert_eq!(None, passport.hgt);
  }

  #[test]
  fn when_setting_hgt_cm_non_numeric() {
    let mut passport = Passport::new();
    assert!(passport.set_hgt("foocm").is_err());
    assert_eq!(None, passport.hgt);
  }

  #[test]
  fn when_setting_hgt_cm_too_low() {
    let mut passport = Passport::new();
    assert!(passport.set_hgt("149cm").is_err());
    assert_eq!(None, passport.hgt);
  }

  #[test]
  fn when_setting_hgt_cm_too_high() {
    let mut passport = Passport::new();
    assert!(passport.set_hgt("194cm").is_err());
    assert_eq!(None, passport.hgt);
  }

  #[test]
  fn when_setting_hgt_60in() -> Result<(), String> {
    let mut passport = Passport::new();
    passport.set_hgt("60in")?;

    assert_eq!(Some("60in".into()), passport.hgt);
    Ok(())
  }

  #[test]
  fn when_setting_hgt_in_non_numeric() {
    let mut passport = Passport::new();
    assert!(passport.set_hgt("fooin").is_err());
    assert_eq!(None, passport.hgt);
  }

  #[test]
  fn when_setting_hgt_in_too_low() {
    let mut passport = Passport::new();
    assert!(passport.set_hgt("58in").is_err());
    assert_eq!(None, passport.hgt);
  }

  #[test]
  fn when_setting_hgt_in_too_high() {
    let mut passport = Passport::new();
    assert!(passport.set_hgt("77in").is_err());
    assert_eq!(None, passport.hgt);
  }
}
