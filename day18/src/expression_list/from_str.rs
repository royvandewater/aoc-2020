use std::str::FromStr;

use crate::expression::Expression;

use super::ExpressionList;

impl FromStr for ExpressionList {
    type Err = String;

    fn from_str(stream: &str) -> Result<Self, Self::Err> {
        let value: Vec<Expression> = stream
            .trim()
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Expression>, String>>()?;

        Ok(ExpressionList(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let sut: ExpressionList = "".parse()?;

        assert_eq!(0, sut.0.len());
        Ok(())
    }
}
