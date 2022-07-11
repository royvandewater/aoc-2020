use std::collections::VecDeque;
use std::str::FromStr;

use super::Instruction;
use super::WaypointShip;

impl FromStr for WaypointShip {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .lines()
            .map(|l| l.trim().parse::<Instruction>())
            .collect::<Result<VecDeque<Instruction>, String>>()?;

        Ok(WaypointShip {
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
        let sut = WaypointShip::from_str(
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
