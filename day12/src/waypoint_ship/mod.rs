mod from_str;

use std::collections::VecDeque;

use crate::instruction::Bearing;
use crate::instruction::Direction;
use crate::instruction::Heading;
use crate::instruction::Instruction;

type Coord = (isize, isize);

#[derive(Debug)]
pub struct WaypointShip {
    instructions: VecDeque<Instruction>,
    position: Coord,
    waypoint: Coord,
}

impl Default for WaypointShip {
    fn default() -> Self {
        Self {
            instructions: VecDeque::new(),
            position: (0, 0),
            waypoint: (10, 1),
        }
    }
}

impl WaypointShip {
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
            Bearing::Left => self.rotate_waypoint_left(amount / 90),
            Bearing::Right => self.rotate_waypoint_right(amount / 90),
            Bearing::Forward => self.go_forward(amount),
        }
    }

    fn rotate_waypoint_left(&mut self, times: isize) {
        match times {
            0 => {}
            n => {
                let (x, y) = self.waypoint;
                self.waypoint = (-y, x);
                self.rotate_waypoint_left(n - 1)
            }
        }
    }

    fn rotate_waypoint_right(&mut self, times: isize) {
        match times {
            0 => {}
            n => {
                let (x, y) = self.waypoint;
                self.waypoint = (y, -x);
                self.rotate_waypoint_right(n - 1)
            }
        }
    }

    fn go_forward(&mut self, amount: isize) {
        let (x, y) = self.position;
        let (wx, wy) = self.waypoint;

        self.position = (x + (wx * amount), y + (wy * amount));
    }

    fn apply_heading(&mut self, heading: Heading, amount: isize) {
        let (x, y) = self.waypoint;

        self.waypoint = match heading {
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
    fn forward_10() -> Result<(), String> {
        let mut sut: WaypointShip = "
            F10
        "
        .parse()?;
        sut.run();

        assert_eq!((100, 10), sut.position);
        Ok(())
    }

    #[test]
    fn north_3() -> Result<(), String> {
        let mut sut: WaypointShip = "
            N2
        "
        .parse()?;
        sut.run();

        assert_eq!((10, 3), sut.waypoint);
        Ok(())
    }

    #[test]
    fn right_90() -> Result<(), String> {
        let mut sut: WaypointShip = "
            R90
        "
        .parse()?;
        sut.run();

        assert_eq!((1, -10), sut.waypoint);
        Ok(())
    }

    #[test]
    fn left_90() -> Result<(), String> {
        let mut sut: WaypointShip = "
            L90
        "
        .parse()?;
        sut.run();

        assert_eq!((-1, 10), sut.waypoint);
        Ok(())
    }

    #[test]
    fn example_1() -> Result<(), String> {
        let mut sut: WaypointShip = "
            F10
            N3
            F7
            R90
            F11
        "
        .parse()?;
        sut.run();
        assert_eq!((214, -72), sut.position);
        assert_eq!(286, sut.manhattan_distance_from_origin());

        Ok(())
    }
}
