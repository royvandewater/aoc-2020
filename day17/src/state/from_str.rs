use std::{collections::HashMap, str::FromStr};

use super::{Position, State};

impl FromStr for State {
    type Err = String;

    fn from_str(stream: &str) -> Result<Self, Self::Err> {
        let value: HashMap<Position, bool> = stream
            .trim()
            .lines()
            .enumerate()
            .flat_map(parse_line)
            .collect();

        Ok(State(value))
    }
}

fn parse_line((row_number, line): (usize, &str)) -> HashMap<Position, bool> {
    line.trim()
        .chars()
        .enumerate()
        .map(|(i, c)| ((i as isize, row_number as isize, 0, 0), c == '#'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let sut: State = "".parse()?;

        assert_eq!(0, sut.0.len());
        Ok(())
    }

    #[test]
    fn test_one_active() -> Result<(), String> {
        let sut: State = "#".parse()?;

        assert_eq!(1, sut.0.len());
        Ok(())
    }

    #[test]
    fn test_two_rows() -> Result<(), String> {
        let sut: State = "
        .#
        ..
        "
        .parse()?;

        assert_eq!(4, sut.0.len());
        assert_eq!(&false, sut.0.get(&(0, 0, 0, 0)).unwrap());
        assert_eq!(&true, sut.0.get(&(1, 0, 0, 0)).unwrap());
        assert_eq!(&false, sut.0.get(&(0, 1, 0, 0)).unwrap());
        assert_eq!(&false, sut.0.get(&(1, 1, 0, 0)).unwrap());
        Ok(())
    }
}
