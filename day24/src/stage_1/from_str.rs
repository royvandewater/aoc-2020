use std::str::FromStr;

use crate::input::Input;

use super::Stage1;

impl FromStr for Stage1 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Input = s.trim().parse()?;

        Ok(Stage1::from(&input))
    }
}
