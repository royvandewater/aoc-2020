mod instruction;

use std::{slice::Iter, str::FromStr};

use self::instruction::Instruction;

pub struct Input(Vec<Instruction>);

impl Input {
    pub fn iter(&self) -> Iter<Instruction> {
        self.0.iter()
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<Vec<Instruction>, String>>()?;

        Ok(Input(instructions))
    }
}

#[cfg(test)]
mod tests {
    use super::{instruction::Direction, *};
    use Direction::*;

    #[test]
    fn test_empty() {
        let sut: Input = "".parse().unwrap();

        assert_eq!(0, sut.iter().count());
    }

    #[test]
    fn test_single_line() {
        let sut: Input = "esenee".parse().unwrap();
        let actual: Vec<Instruction> = sut.iter().cloned().collect();

        assert_eq!(
            vec![Instruction::from([East, SouthEast, NorthEast, East])],
            actual
        )
    }
}
