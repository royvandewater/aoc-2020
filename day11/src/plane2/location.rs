#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Location {
    Floor,
    Empty,
    Occupied,
}

impl TryFrom<char> for Location {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Location::Floor),
            'L' => Ok(Location::Empty),
            '#' => Ok(Location::Occupied),
            _ => Err(format!("Unrecognized location: {}", c)),
        }
    }
}
