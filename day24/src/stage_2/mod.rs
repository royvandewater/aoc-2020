mod from;
mod from_str;
mod simulate_day;

use simulate_day::simulate_day;
use std::collections::HashSet;

type Position = (isize, isize, isize);

pub struct Stage2(HashSet<Position>);

impl Stage2 {
    pub fn answer(&self) -> usize {
        (0..100).fold(self.0.clone(), simulate_day).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input::Direction;
    use crate::input::Direction::*;

    #[test]
    fn test_empty() {
        let sut: Stage2 = Stage2::from([] as [[Direction; 0]; 0]);
        assert_eq!(0, sut.answer());
    }

    #[test]
    fn test_single_instruction() {
        let sut: Stage2 = Stage2::from([[East]]);
        assert_eq!(0, sut.answer());
    }

    #[test]
    fn test_example_1() {
        let sut: Stage2 = "
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

        assert_eq!(2208, sut.answer());
    }
}
