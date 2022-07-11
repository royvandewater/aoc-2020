mod from_str;
mod try_from;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Heading {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
pub enum Bearing {
    Left,
    Right,
    Forward,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Bearing(Bearing),
    Heading(Heading),
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub direction: Direction,
    pub amount: isize,
}
