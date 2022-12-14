use std::collections::HashSet;

use crate::input::{Direction, Instruction};

mod from;
mod from_str;

pub struct Stage1(Vec<Instruction>);

type Position = (isize, isize);

impl Stage1 {
    pub fn answer(&self) -> usize {
        let black_tiles = self
            .0
            .iter()
            .fold(HashSet::<Position>::new(), apply_instruction);
        return black_tiles.len();
    }
}

fn apply_instruction(
    mut black_tiles: HashSet<Position>,
    instruction: &Instruction,
) -> HashSet<Position> {
    let position: Position = resolve_position(instruction);

    match black_tiles.contains(&position) {
        true => black_tiles.remove(&position),
        false => black_tiles.insert(position),
    };

    return black_tiles;
}

fn resolve_position(instruction: &Instruction) -> Position {
    instruction.iter().fold((0, 0), apply_direction)
}

fn apply_direction(position: Position, &direction: &Direction) -> Position {
    let (x, y) = position;

    match direction {
        Direction::East => (x + 1, y),
        Direction::SouthEast => (x, y - 1),
        Direction::SouthWest => (x - 1, y - 1),
        Direction::West => (x - 1, y),
        Direction::NorthWest => (x - 1, y + 1),
        Direction::NorthEast => (x, y + 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Direction::{self, *};

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
}
