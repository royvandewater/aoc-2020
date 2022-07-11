use std::collections::VecDeque;
use std::str::FromStr;

use super::Instruction;
use super::Ship;

impl FromStr for Ship {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .lines()
            .map(|l| l.parse::<Instruction>())
            .collect::<Result<VecDeque<Instruction>, String>>()?;

        Ok(Ship {
            instructions,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::instruction::*;

    #[test]
    fn north_3() -> Result<(), String> {
        let sut = Ship::from_str(
            "
                N3
                L9
            ",
        )?;

        assert_eq!(
            VecDeque::from(vec![
                Instruction {
                    direction: Direction::Heading(Heading::North),
                    amount: 3
                },
                Instruction {
                    direction: Direction::Bearing(Bearing::Left),
                    amount: 9
                },
            ]),
            sut.instructions
        );
        Ok(())
    }
}
