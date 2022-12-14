use std::collections::HashSet;

use crate::input::{Direction, Instruction};

mod from;
mod from_str;

pub struct Stage1(Vec<Instruction>);

type Position = (isize, isize, isize);

impl Stage1 {
    pub fn answer(&self) -> usize {
        let black_tiles = self
            .0
            .iter()
            .map(resolve_position)
            .fold(HashSet::<Position>::new(), apply_position);

        return black_tiles.len();
    }
}

fn apply_position(mut black_tiles: HashSet<Position>, position: Position) -> HashSet<Position> {
    match black_tiles.contains(&position) {
        true => black_tiles.remove(&position),
        false => black_tiles.insert(position.clone()),
    };

    return black_tiles;
}

fn resolve_position(instruction: &Instruction) -> Position {
    instruction.iter().fold((0, 0, 0), apply_direction)
}

fn apply_direction(position: Position, &direction: &Direction) -> Position {
    let (q, r, s) = position;

    match direction {
        Direction::East => (q + 1, r, s - 1),
        Direction::SouthEast => (q, r + 1, s - 1),
        Direction::SouthWest => (q - 1, r + 1, s),
        Direction::West => (q - 1, r, s + 1),
        Direction::NorthWest => (q, r - 1, s + 1),
        Direction::NorthEast => (q + 1, r - 1, s),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::input::{
        Direction::{self, *},
        Input,
    };

    #[test]
    fn test_empty() {
        let sut: Stage1 = Stage1::from([] as [[Direction; 0]; 0]);

        assert_eq!(0, sut.answer())
    }

    #[test]
    fn test_one_instruction() {
        let sut: Stage1 = Stage1::from([vec![East, SouthEast, NorthEast, East]]);

        assert_eq!(1, sut.answer())
    }

    #[test]
    fn test_two_instructions() {
        let sut: Stage1 = Stage1::from([vec![East, SouthEast, NorthEast, East], vec![West]]);

        assert_eq!(2, sut.answer())
    }

    #[test]
    fn test_two_instructions_that_resolve_to_the_same_tile() {
        let sut: Stage1 = Stage1::from([vec![East, SouthWest], vec![SouthEast]]);

        assert_eq!(0, sut.answer())
    }

    #[test]
    fn test_example_1() {
        let sut: Stage1 = "
            sesenwnenenewseeswwswswwnenewsewsw
            neeenesenwnwwswnenewnwwsewnenwseswesw
            seswneswswsenwwnwse
            nwnwneseeswswnenewneswwnewseswneseene
            swweswneswnenwsewnwneneseenw
            eesenwseswswnenwswnwnwsewwnwsene
            sewnenenenesenwsewnenwwwse
            wenwwweseeeweswwwnwwe
            wsweesenenewnwwnwsenewsenwwsesesenwne
            neeswseenwwswnwswswnw
            nenwswwsewswnenenewsenwsenwnesesenew
            enewnwewneswsewnwswenweswnenwsenwsw
            sweneswneswneneenwnewenewwneswswnese
            swwesenesewenwneswnwwneseswwne
            enesenwswwswneneswsenwnewswseenwsese
            wnwnesenesenenwwnenwsewesewsesesew
            nenewswnwewswnenesenwnesewesw
            eneswnwswnwsenenwnwnwwseeswneewsenese
            neswnwewnwnwseenwseesewsenwsweewe
            wseweeenwnesenwwwswnew"
            .parse()
            .unwrap();

        assert_eq!(10, sut.answer())
    }

    #[test]
    fn test_puzzle_input() {
        let input_str = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
        let input: Input = input_str.parse().expect("could not parse input as Input");
        let sut: Stage1 = input.into();

        assert!(315 < sut.answer(), "assert! 315 < '{}'", sut.answer());
        assert!(sut.answer() < 470, "assert! '{}' < 470 ", sut.answer());
    }
}
