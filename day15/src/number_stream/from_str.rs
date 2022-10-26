use std::{num::ParseIntError, str::FromStr};

use super::NumberStream;

impl FromStr for NumberStream {
    type Err = String;

    fn from_str(stream: &str) -> Result<Self, Self::Err> {
        let numbers = stream
            .trim()
            .split(",")
            .filter(|s| !s.is_empty())
            .map(usize::from_str)
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .map_err(|e| format!("Failed to parse value as usize: {}", e))?;

        Ok(NumberStream(numbers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let sut: NumberStream = "".parse()?;

        assert_eq!(0, sut.0.len());
        Ok(())
    }

    #[test]
    fn test_3_numbers() -> Result<(), String> {
        let sut: NumberStream = "
            0,0,0
        "
        .parse()?;

        assert_eq!(3, sut.0.len());
        Ok(())
    }

    #[test]
    fn test_6_numbers() -> Result<(), String> {
        let sut: NumberStream = "
          0,13,1,8,6,15
        "
        .parse()?;

        assert_eq!(6, sut.0.len());
        Ok(())
    }
}
