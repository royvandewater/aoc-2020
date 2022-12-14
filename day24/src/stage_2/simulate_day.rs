use std::collections::HashSet;

type Position = (isize, isize, isize);

pub fn simulate_day(black_tiles: HashSet<Position>, _: usize) -> HashSet<Position> {
    let neighbors_of_black_tiles: HashSet<Position> =
        black_tiles.iter().flat_map(find_neighbors).collect();

    let white_tiles: HashSet<Position> = neighbors_of_black_tiles
        .difference(&black_tiles)
        .cloned()
        .collect();

    let retained_black_tiles: HashSet<Position> = black_tiles
        .iter()
        .filter(|position| tile_stays_black(&black_tiles, position))
        .cloned()
        .collect();

    let new_black_tiles: HashSet<Position> = white_tiles
        .iter()
        .filter(|position| white_tile_turns_black(&black_tiles, position))
        .cloned()
        .collect();

    retained_black_tiles
        .union(&new_black_tiles)
        .cloned()
        .collect()
}

fn tile_stays_black(black_tiles: &HashSet<Position>, position: &Position) -> bool {
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

fn white_tile_turns_black(black_tiles: &HashSet<Position>, position: &Position) -> bool {
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

fn find_neighbors(position: &Position) -> HashSet<Position> {
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
        let input = HashSet::new();
        let output = simulate_day(input, IGNORED);

        assert_eq!(HashSet::new(), output);
    }

    #[test]
    fn test_one_tile() {
        let input = HashSet::from([(0, 0, 0)]);
        let output = simulate_day(input, IGNORED);

        assert_eq!(HashSet::new(), output);
    }

    #[test]
    fn test_two_neighboring_tiles() {
        let input = HashSet::from([(0, 0, 0), (-1, 0, 1)]);
        let output = simulate_day(input, IGNORED);

        assert_eq!(
            HashSet::from([(0, 0, 0), (-1, 0, 1), (0, -1, 1), (-1, 1, 0)]),
            output
        );
    }
}
