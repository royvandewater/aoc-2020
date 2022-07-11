use std::str::FromStr;

use super::*;

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.trim().chars();
        let direction_char = chars.next().ok_or("Could not access direction char")?;
        let direction = Direction::try_from(direction_char)?;
        let amount: isize = chars
            .as_str()
            .parse()
            .map_err(|e| format!("Failed to parse amount: {}", e))?;

        Ok(Instruction { direction, amount })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn north_3() -> Result<(), String> {
        let sut = Instruction::from_str("N3")?;

        assert_eq!(Direction::Heading(Heading::North), sut.direction);
        assert_eq!(3, sut.amount);
        Ok(())
    }

    #[test]
    fn south_10() -> Result<(), String> {
        let sut = Instruction::from_str("S10")?;

        assert_eq!(Direction::Heading(Heading::South), sut.direction);
        assert_eq!(10, sut.amount);
        Ok(())
    }

    #[test]
    fn east_9() -> Result<(), String> {
        let sut = Instruction::from_str("E9")?;

        assert_eq!(Direction::Heading(Heading::East), sut.direction);
        assert_eq!(9, sut.amount);
        Ok(())
    }

    #[test]
    fn west_1() -> Result<(), String> {
        let sut = Instruction::from_str("W1")?;

        assert_eq!(Direction::Heading(Heading::West), sut.direction);
        assert_eq!(1, sut.amount);
        Ok(())
    }

    #[test]
    fn left_1() -> Result<(), String> {
        let sut = Instruction::from_str("L1")?;

        assert_eq!(Direction::Bearing(Bearing::Left), sut.direction);
        assert_eq!(1, sut.amount);
        Ok(())
    }

    #[test]
    fn right_5() -> Result<(), String> {
        let sut = Instruction::from_str("R5")?;

        assert_eq!(Direction::Bearing(Bearing::Right), sut.direction);
        assert_eq!(5, sut.amount);
        Ok(())
    }
}
