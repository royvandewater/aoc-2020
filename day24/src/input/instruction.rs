use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction(Vec<Direction>);

const TWO_CHAR_DIRECTIONS: [&str; 4] = ["se", "sw", "nw", "ne"];

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.trim().to_string();
        let mut directions = Vec::<Direction>::new();

        while !input.is_empty() {
            let num_chars = match TWO_CHAR_DIRECTIONS.iter().any(|d| input.starts_with(d)) {
                true => 2,
                false => 1,
            };

            let (d_str, rest) = input.split_at(num_chars);

            let direction = match d_str {
                "e" => Direction::East,
                "se" => Direction::SouthEast,
                "sw" => Direction::SouthWest,
                "w" => Direction::West,
                "nw" => Direction::NorthWest,
                "ne" => Direction::NorthEast,
                _ => return Err(format!("Unrecognized direction: {}", d_str)),
            };

            directions.push(direction);
            input = rest.to_string();
        }

        return Ok(Instruction(directions));
    }
}

impl From<Vec<Direction>> for Instruction {
    fn from(directions: Vec<Direction>) -> Self {
        Instruction(directions)
    }
}

impl<const N: usize> From<[Direction; N]> for Instruction {
    fn from(directions: [Direction; N]) -> Self {
        Instruction(Vec::from(directions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    #[test]
    fn test_empty() {
        let actual: Instruction = "".parse().unwrap();

        assert_eq!(Instruction::from([]), actual);
    }

    #[test]
    fn test_single_line() {
        let actual: Instruction = "esenee".parse().unwrap();

        assert_eq!(
            Instruction::from([East, SouthEast, NorthEast, East]),
            actual
        )
    }
}
