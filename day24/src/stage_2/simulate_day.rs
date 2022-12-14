use rustc_hash::FxHashSet;

type Position = (isize, isize, isize);

pub fn simulate_day(black_tiles: FxHashSet<Position>, _: usize) -> FxHashSet<Position> {
    let neighbors_of_black_tiles: FxHashSet<Position> =
        black_tiles.iter().flat_map(find_neighbors).collect();

    let white_tiles: FxHashSet<Position> = neighbors_of_black_tiles
        .difference(&black_tiles)
        .cloned()
        .collect();

    let retained_black_tiles: FxHashSet<Position> = black_tiles
        .iter()
        .filter(|position| tile_stays_black(&black_tiles, position))
        .cloned()
        .collect();

    let new_black_tiles: FxHashSet<Position> = white_tiles
        .iter()
        .filter(|position| white_tile_turns_black(&black_tiles, position))
        .cloned()
        .collect();

    retained_black_tiles
        .union(&new_black_tiles)
        .cloned()
        .collect()
}

fn tile_stays_black(black_tiles: &FxHashSet<Position>, position: &Position) -> bool {
    let num_black_tiles_adjacent = find_neighbors(position)
        .iter()
        .filter(|neighbor| black_tiles.contains(neighbor))
        .count();

    match num_black_tiles_adjacent {
        1 => true,
        2 => true,
        _ => false,
    }
}

fn white_tile_turns_black(black_tiles: &FxHashSet<Position>, position: &Position) -> bool {
    let num_black_tiles_adjacent = find_neighbors(position)
        .iter()
        .filter(|neighbor| black_tiles.contains(neighbor))
        .count();

    match num_black_tiles_adjacent {
        2 => true,
        _ => false,
    }
}

const NEIGHBOR_OFFSETS: [Position; 6] = [
    (1, 0, -1),
    (0, 1, -1),
    (-1, 1, 0),
    (-1, 0, 1),
    (0, -1, 1),
    (1, -1, 0),
];

fn find_neighbors(position: &Position) -> FxHashSet<Position> {
    NEIGHBOR_OFFSETS
        .iter()
        .map(|offset| apply_offset(position, offset))
        .collect()
}

fn apply_offset(position: &Position, offset: &Position) -> Position {
    let (pr, pq, ps) = position;
    let (dr, dq, ds) = offset;

    (pr + dr, pq + dq, ps + ds)
}

#[cfg(test)]
mod tests {
    use super::*;

    const IGNORED: usize = 0;

    #[test]
    fn test_empty() {
        let input = FxHashSet::default();
        let output = simulate_day(input, IGNORED);

        assert_eq!(FxHashSet::default(), output);
    }

    #[test]
    fn test_one_tile() {
        let mut input = FxHashSet::default();
        input.insert((0, 0, 0));

        let output = simulate_day(input, IGNORED);

        assert_eq!(FxHashSet::default(), output);
    }

    #[test]
    fn test_two_neighboring_tiles() {
        let mut input = FxHashSet::default();
        input.insert((0, 0, 0));
        input.insert((-1, 0, 1));

        let output = simulate_day(input, IGNORED);

        let mut expected = FxHashSet::default();
        expected.insert((0, 0, 0));
        expected.insert((-1, 0, 1));
        expected.insert((0, -1, 1));
        expected.insert((-1, 1, 0));

        assert_eq!(expected, output);
    }
}
