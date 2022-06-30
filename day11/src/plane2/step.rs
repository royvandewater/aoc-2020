use super::Location;
use super::Plane2;

impl Plane2 {
    pub fn step(&mut self) {
        let mut next = self.grid.clone();

        for (i, row) in self.grid.iter().enumerate() {
            for (j, location) in row.iter().enumerate() {
                next[i][j] = match location {
                    Location::Empty if self.count_adjacent_seats_occupied(i, j) == 0 => {
                        Location::Occupied
                    }
                    Location::Occupied if self.count_adjacent_seats_occupied(i, j) >= 5 => {
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

    fn vector_occupied(&self, (i, j): (usize, usize), (di, dj): (isize, isize)) -> bool {
        if i == 0 && di < 0 {
            return false;
        }
        if i == self.grid.len() - 1 && di > 0 {
            return false;
        }
        let ii: isize = i.try_into().unwrap();
        let i2: usize = (ii + di).try_into().unwrap();

        let row = &self.grid[i2];
        if j == 0 && dj < 0 {
            return false;
        }
        if j == row.len() - 1 && dj > 0 {
            return false;
        }

        let jj: isize = j.try_into().unwrap();
        let j2: usize = (jj + dj).try_into().unwrap();

        match row[j2] {
            Location::Occupied => true,
            Location::Empty => false,
            Location::Floor => self.vector_occupied((i2, j2), (di, dj)),
        }
    }

    fn count_previous_row_occupied(&self, i: usize, j: usize) -> usize {
        let mut count = 0;

        if self.vector_occupied((i, j), (-1, -1)) {
            count += 1;
        }
        if self.vector_occupied((i, j), (-1, 0)) {
            count += 1;
        }
        if self.vector_occupied((i, j), (-1, 1)) {
            count += 1;
        }

        return count;
    }

    fn count_current_row_occupied(&self, i: usize, j: usize) -> usize {
        let mut count = 0;

        if self.vector_occupied((i, j), (0, -1)) {
            count += 1;
        }
        if self.vector_occupied((i, j), (0, 1)) {
            count += 1;
        }

        return count;
    }

    fn count_next_row_occupied(&self, i: usize, j: usize) -> usize {
        let mut count = 0;

        if self.vector_occupied((i, j), (1, -1)) {
            count += 1;
        }
        if self.vector_occupied((i, j), (1, 0)) {
            count += 1;
        }
        if self.vector_occupied((i, j), (1, 1)) {
            count += 1;
        }

        return count;
    }
}

#[cfg(test)]
mod tests {
    use crate::plane2::from_str::parse_grid;

    use super::super::Grid;
    use super::*;

    #[test]
    fn empty_grid() {
        let mut sut = Plane2 { grid: vec![] };
        sut.step();
        assert_eq!(vec![] as Grid, sut.grid);
    }

    #[test]
    fn one_empty_seat() {
        let mut sut = Plane2 {
            grid: vec![vec![Location::Empty]],
        };
        sut.step();
        assert_eq!(vec![vec![Location::Occupied]], sut.grid);
    }

    #[test]
    fn occupied_seat_touching_4_occupied_seats_becomes_empty() {
        let mut sut = Plane2 {
            grid: vec![
                vec![Location::Occupied, Location::Occupied, Location::Occupied],
                vec![Location::Floor, Location::Occupied, Location::Floor],
                vec![Location::Occupied, Location::Floor, Location::Occupied],
            ],
        };
        sut.step();

        assert_eq!(
            vec![
                vec![Location::Occupied, Location::Occupied, Location::Occupied],
                vec![Location::Floor, Location::Empty, Location::Floor],
                vec![Location::Occupied, Location::Floor, Location::Occupied],
            ],
            sut.grid
        );
    }

    #[test]
    fn example_1_step_1() -> Result<(), String> {
        let mut sut: Plane2 = "
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
        let mut sut: Plane2 = "
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
            #.LL.LL.L#
            #LLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLL#
            #.LLLLLL.L
            #.LLLLL.L#
            ",
        )?;

        sut.step();
        assert_eq!(expected, sut.grid);

        Ok(())
    }

    #[test]
    fn example_1_step_3() -> Result<(), String> {
        let mut sut: Plane2 = "
            #.LL.LL.L#
            #LLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLL#
            #.LLLLLL.L
            #.LLLLL.L#
        "
        .parse()?;

        let expected: Grid = parse_grid(
            "
            #.L#.##.L#
            #L#####.LL
            L.#.#..#..
            ##L#.##.##
            #.##.#L.##
            #.#####.#L
            ..#.#.....
            LLL####LL#
            #.L#####.L
            #.L####.L#
            ",
        )?;

        sut.step();
        assert_eq!(expected, sut.grid);

        Ok(())
    }

    #[test]
    fn example_1_step_4() -> Result<(), String> {
        let mut sut: Plane2 = "
            #.L#.##.L#
            #L#####.LL
            L.#.#..#..
            ##L#.##.##
            #.##.#L.##
            #.#####.#L
            ..#.#.....
            LLL####LL#
            #.L#####.L
            #.L####.L#
        "
        .parse()?;

        let expected: Grid = parse_grid(
            "
            #.L#.L#.L#
            #LLLLLL.LL
            L.L.L..#..
            ##LL.LL.L#
            L.LL.LL.L#
            #.LLLLL.LL
            ..L.L.....
            LLLLLLLLL#
            #.LLLLL#.L
            #.L#LL#.L#
            ",
        )?;

        sut.step();
        assert_eq!(expected, sut.grid);

        Ok(())
    }
}
