mod from;
mod validation;

#[derive(Debug, PartialEq)]
pub struct Passport {
  ecl: Option<String>,
  eyr: Option<u16>,
  hcl: Option<String>,
  pid: Option<String>,
  byr: Option<u16>,
  iyr: Option<u16>,
  hgt: Option<String>,
}

impl Passport {
  pub fn new() -> Passport {
    Passport {
      ecl: None,
      pid: None,
      eyr: None,
      hcl: None,
      byr: None,
      iyr: None,
      hgt: None,
    }
  }
}
