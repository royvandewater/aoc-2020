mod from_str;
mod location;
mod step;

pub use location::Location;

pub type Grid = Vec<Vec<Location>>;

pub struct Plane {
    grid: Grid,
}

impl Plane {
    pub fn stabilize(&mut self) {
        loop {
            let previous = self.grid.clone();

            self.step();

            if previous == self.grid {
                return;
            }
        }
    }

    pub fn count_occupied_seats(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|loc| **loc == Location::Occupied).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> Result<(), String> {
        let mut sut: Plane = "
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL
        "
        .parse()?;

        sut.stabilize();
        assert_eq!(37, sut.count_occupied_seats());

        Ok(())
    }
}
