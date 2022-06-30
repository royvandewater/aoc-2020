use super::Location;
use super::Plane;

impl Plane {
    pub fn step(&mut self) {
        let mut next = self.grid.clone();

        for (i, row) in self.grid.iter().enumerate() {
            for (j, location) in row.iter().enumerate() {
                next[i][j] = match location {
                    Location::Empty if self.count_adjacent_seats_occupied(i, j) == 0 => {
                        Location::Occupied
                    }
                    Location::Occupied if self.count_adjacent_seats_occupied(i, j) >= 4 => {
                        Location::Empty
                    }
                    _ => *location,
                }
            }
        }

        self.grid = next;
    }

    fn count_adjacent_seats_occupied(&self, i: usize, j: usize) -> usize {
        self.count_previous_row_occupied(i, j)
            + self.count_current_row_occupied(i, j)
            + self.count_next_row_occupied(i, j)
    }

    fn count_previous_row_occupied(&self, i: usize, j: usize) -> usize {
        if i == 0 {
            return 0;
        }

        let row = &self.grid[i - 1];
        let max = row.len() - 1;
        let start = if j == 0 { 0 } else { j - 1 };
        let end = if j == max { max } else { j + 1 };

        row[start..=end]
            .iter()
            .filter(|l| **l == Location::Occupied)
            .count()
    }

    fn count_current_row_occupied(&self, i: usize, j: usize) -> usize {
        let row = &self.grid[i];
        let max = row.len() - 1;

        let mut count = 0;

        if j > 0 && row[j - 1] == Location::Occupied {
            count += 1;
        }
        if j < max && row[j + 1] == Location::Occupied {
            count += 1;
        }

        count
    }

    fn count_next_row_occupied(&self, i: usize, j: usize) -> usize {
        if i == self.grid.len() - 1 {
            return 0;
        }

        let row = &self.grid[i + 1];
        let max = row.len() - 1;
        let start = if j == 0 { 0 } else { j - 1 };
        let end = if j == max { max } else { j + 1 };

        row[start..=end]
            .iter()
            .filter(|l| **l == Location::Occupied)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::plane::from_str::parse_grid;

    use super::super::Grid;
    use super::*;

    #[test]
    fn empty_grid() {
        let mut sut = Plane { grid: vec![] };
        sut.step();
        assert_eq!(vec![] as Grid, sut.grid);
    }

    #[test]
    fn one_empty_seat() {
        let mut sut = Plane {
            grid: vec![vec![Location::Empty]],
        };
        sut.step();
        assert_eq!(vec![vec![Location::Occupied]], sut.grid);
    }

    #[test]
    fn occupied_seat_touching_4_occupied_seats_becomes_empty() {
        let mut sut = Plane {
            grid: vec![
                vec![Location::Occupied, Location::Floor, Location::Occupied],
                vec![Location::Floor, Location::Occupied, Location::Floor],
                vec![Location::Occupied, Location::Floor, Location::Occupied],
            ],
        };
        sut.step();

        assert_eq!(
            vec![
                vec![Location::Occupied, Location::Floor, Location::Occupied],
                vec![Location::Floor, Location::Empty, Location::Floor],
                vec![Location::Occupied, Location::Floor, Location::Occupied],
            ],
            sut.grid
        );
    }

    #[test]
    fn example_1_step_1() -> Result<(), String> {
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

        let expected: Grid = parse_grid(
            "
            #.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##
            ",
        )?;

        sut.step();
        assert_eq!(expected, sut.grid);

        Ok(())
    }

    #[test]
    fn example_1_step_2() -> Result<(), String> {
        let mut sut: Plane = "
            #.##.##.##
            #######.##
            #.#.#..#..
            ####.##.##
            #.##.##.##
            #.#####.##
            ..#.#.....
            ##########
            #.######.#
            #.#####.##
        "
        .parse()?;

        let expected: Grid = parse_grid(
            "
            #.LL.L#.##
            #LLLLLL.L#
            L.L.L..L..
            #LLL.LL.L#
            #.LL.LL.LL
            #.LLLL#.##
            ..L.L.....
            #LLLLLLLL#
            #.LLLLLL.L
            #.#LLLL.##
            ",
        )?;

        sut.step();
        assert_eq!(expected, sut.grid);

        Ok(())
    }

    #[test]
    fn example_1_step_3() -> Result<(), String> {
        let mut sut: Plane = "
            #.LL.L#.##
            #LLLLLL.L#
            L.L.L..L..
            #LLL.LL.L#
            #.LL.LL.LL
            #.LLLL#.##
            ..L.L.....
            #LLLLLLLL#
            #.LLLLLL.L
            #.#LLLL.##
        "
        .parse()?;

        let expected: Grid = parse_grid(
            "
            #.##.L#.##
            #L###LL.L#
            L.#.#..#..
            #L##.##.L#
            #.##.LL.LL
            #.###L#.##
            ..#.#.....
            #L######L#
            #.LL###L.L
            #.#L###.##
            ",
        )?;

        sut.step();
        assert_eq!(expected, sut.grid);

        Ok(())
    }

    #[test]
    fn example_1_step_4() -> Result<(), String> {
        let mut sut: Plane = "
            #.##.L#.##
            #L###LL.L#
            L.#.#..#..
            #L##.##.L#
            #.##.LL.LL
            #.###L#.##
            ..#.#.....
            #L######L#
            #.LL###L.L
            #.#L###.##
        "
        .parse()?;

        let expected: Grid = parse_grid(
            "
            #.#L.L#.##
            #LLL#LL.L#
            L.L.L..#..
            #LLL.##.L#
            #.LL.LL.LL
            #.LL#L#.##
            ..L.L.....
            #L#LLLL#L#
            #.LLLLLL.L
            #.#L#L#.##
            ",
        )?;

        sut.step();
        assert_eq!(expected, sut.grid);

        Ok(())
    }
}
