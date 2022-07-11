use super::*;

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' => Ok(Direction::Heading(Heading::North)),
            'S' => Ok(Direction::Heading(Heading::South)),
            'E' => Ok(Direction::Heading(Heading::East)),
            'W' => Ok(Direction::Heading(Heading::West)),
            'L' => Ok(Direction::Bearing(Bearing::Left)),
            'R' => Ok(Direction::Bearing(Bearing::Right)),
            'F' => Ok(Direction::Bearing(Bearing::Forward)),
            _ => Err(format!("Failed to parse heading: {}", value)),
        }
    }
}
