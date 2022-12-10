mod food;

use std::{slice::Iter, str::FromStr};

pub use food::Food;

pub struct Input(Vec<Food>);

impl Input {
    pub fn iter(&self) -> Iter<Food> {
        self.0.iter()
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Food> = s
            .trim()
            .lines()
            .map(Food::from_str)
            .collect::<Result<Vec<Food>, String>>()?;

        Ok(Input(lines))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let sut: Input = "".parse()?;
        assert_eq!(0, sut.iter().count());
        Ok(())
    }

    #[test]
    fn test_example_1() -> Result<(), String> {
        let sut: Input = "
            mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
            trh fvjkl sbzzf mxmxvkd (contains dairy)
            sqjhc fvjkl (contains soy)
            sqjhc mxmxvkd sbzzf (contains fish)
        "
        .parse()?;

        assert_eq!(4, sut.iter().count());
        Ok(())
    }
}
