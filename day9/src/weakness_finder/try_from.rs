use std::num::ParseIntError;

use super::WeaknessFinder;

impl TryFrom<Vec<&str>> for WeaknessFinder {
    type Error = String;

    fn try_from(sequence_strs: Vec<&str>) -> Result<Self, Self::Error> {
        let sequence = sequence_strs
            .iter()
            .map(|s| usize::from_str_radix(s, 10))
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .or(Err("Failed to parse preamble"))?;

        Ok(WeaknessFinder { sequence })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sequence() -> Result<(), String> {
        let sut = WeaknessFinder::try_from(vec![])?;
        let expected: Vec<usize> = vec![];
        assert_eq!(expected, sut.sequence);
        Ok(())
    }

    #[test]
    fn valid_numbers() -> Result<(), String> {
        let sut = WeaknessFinder::try_from(vec!["1", "2", "3"])?;

        assert_eq!(vec![1, 2, 3], sut.sequence);
        Ok(())
    }

    #[test]
    fn invalid_numbers() {
        let res = WeaknessFinder::try_from(vec!["a", "b", "c"]);

        assert!(res.is_err())
    }
}
