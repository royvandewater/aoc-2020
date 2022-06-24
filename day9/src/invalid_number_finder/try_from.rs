use std::{collections::VecDeque, num::ParseIntError};

use super::InvalidNumberFinder;

impl TryFrom<(Vec<&str>, Vec<&str>)> for InvalidNumberFinder {
    type Error = String;

    fn try_from((preamble_strs, input_strs): (Vec<&str>, Vec<&str>)) -> Result<Self, Self::Error> {
        let preamble = preamble_strs
            .iter()
            .map(|s| usize::from_str_radix(s, 10))
            .collect::<Result<VecDeque<usize>, ParseIntError>>()
            .or(Err("Failed to parse preamble"))?;

        if preamble.len() < 2 {
            return Err(format!(
                "preamble must contain at least 2 elements, contains {}",
                preamble.len()
            ));
        }

        let input = input_strs
            .iter()
            .map(|s| usize::from_str_radix(s, 10))
            .collect::<Result<VecDeque<usize>, ParseIntError>>()
            .or(Err("Failed to parse rest"))?;

        Ok(InvalidNumberFinder { preamble, input })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_preamble() {
        let preamble: Vec<&str> = vec![];
        let input: Vec<&str> = vec![];

        let res = InvalidNumberFinder::try_from((preamble, input));

        res.expect_err("Expecting error about the preamble too small");
    }

    #[test]
    fn valid_numbers() -> Result<(), String> {
        let preamble: Vec<&str> = vec!["1", "2", "3"];
        let rest: Vec<&str> = vec!["4", "5", "6"];

        let seq = InvalidNumberFinder::try_from((preamble, rest))?;

        assert_eq!(VecDeque::from([1, 2, 3]), seq.preamble);
        assert_eq!(VecDeque::from([4, 5, 6]), seq.input);
        Ok(())
    }

    #[test]
    fn invalid_numbers() {
        let preamble: Vec<&str> = vec!["a", "b", "c"];
        let rest: Vec<&str> = vec!["e", "f", "g"];

        let res = InvalidNumberFinder::try_from((preamble, rest));

        assert!(res.is_err())
    }
}
