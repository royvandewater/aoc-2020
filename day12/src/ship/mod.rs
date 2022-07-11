mod from_str;

use std::collections::VecDeque;

use crate::instruction::Bearing;
use crate::instruction::Direction;
use crate::instruction::Heading;
use crate::instruction::Instruction;

#[derive(Debug)]
pub struct Ship {
    instructions: VecDeque<Instruction>,
    position: (isize, isize),
    heading: Heading,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            instructions: VecDeque::new(),
            position: (0, 0),
            heading: Heading::East,
        }
    }
}

impl Ship {
    pub fn run(&mut self) {
        match self.instructions.pop_front() {
            None => return,
            Some(instruction) => {
                match instruction.direction {
                    Direction::Heading(h) => self.apply_heading(h, instruction.amount),
                    Direction::Bearing(b) => self.apply_bearing(b, instruction.amount),
                };
                self.run()
            }
        };
    }

    pub fn manhattan_distance_from_origin(&self) -> isize {
        let (x, y) = self.position;
        x.abs() + y.abs()
    }

    fn apply_bearing(&mut self, bearing: Bearing, amount: isize) {
        match bearing {
            Bearing::Left => self.turn_left(amount / 90),
            Bearing::Right => self.turn_right(amount / 90),
            Bearing::Forward => self.go_forward(amount),
        }
    }

    fn turn_left(&mut self, times: isize) {
        match times {
            0 => {}
            n => {
                self.heading = match self.heading {
                    Heading::North => Heading::West,
                    Heading::West => Heading::South,
                    Heading::South => Heading::East,
                    Heading::East => Heading::North,
                };
                self.turn_left(n - 1)
            }
        }
    }

    fn turn_right(&mut self, times: isize) {
        match times {
            0 => {}
            n => {
                self.heading = match self.heading {
                    Heading::North => Heading::East,
                    Heading::East => Heading::South,
                    Heading::South => Heading::West,
                    Heading::West => Heading::North,
                };
                self.turn_right(n - 1)
            }
        }
    }

    fn go_forward(&mut self, amount: isize) {
        self.apply_heading(self.heading, amount)
    }

    fn apply_heading(&mut self, heading: Heading, amount: isize) {
        let (x, y) = self.position;

        self.position = match heading {
            Heading::North => (x, y + amount),
            Heading::South => (x, y - amount),
            Heading::East => (x + amount, y),
            Heading::West => (x - amount, y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn north_3() -> Result<(), String> {
        let mut sut: Ship = "
            N2
        "
        .parse()?;
        sut.run();
        assert_eq!(2, sut.manhattan_distance_from_origin());

        Ok(())
    }

    #[test]
    fn example_1() -> Result<(), String> {
        let mut sut: Ship = "
            F10
            N3
            F7
            R90
            F11
        "
        .parse()?;
        sut.run();
        assert_eq!(25, sut.manhattan_distance_from_origin());

        Ok(())
    }
}
