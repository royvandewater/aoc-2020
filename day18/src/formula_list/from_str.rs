use std::str::FromStr;

use crate::formula::Formula;

use super::FormulaList;

impl FromStr for FormulaList {
    type Err = String;

    fn from_str(stream: &str) -> Result<Self, Self::Err> {
        let value: Vec<Formula> = stream
            .trim()
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Formula>, String>>()?;

        Ok(FormulaList(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let sut: FormulaList = "".parse()?;

        assert_eq!(0, sut.0.len());
        Ok(())
    }
}
