use std::str::FromStr;

use crate::input::Input;

use super::Stage2;

impl FromStr for Stage2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Input = s.trim().parse()?;

        Ok(Stage2::from(&input))
    }
}
