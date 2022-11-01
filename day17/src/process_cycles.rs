use std::collections::{HashMap, HashSet};

use crate::state::{Position, State};

pub fn count_active_cubes(state: State) -> usize {
    state.iter().filter(|(_, &active)| active).count()
}

pub fn process_cycles(number_of_cycles: i32, begin_state: &State) -> State {
    let mut state: State = begin_state.clone();

    for _ in 0..number_of_cycles {
        state = process_cycle(&state);
    }

    return state;
}

fn process_cycle(state: &State) -> State {
    let known_positions = compute_known_positions(state);
    let mut next_state: HashMap<Position, bool> = HashMap::new();

    for position in known_positions {
        next_state.insert(position, compute_active(&position, &state));
    }

    return next_state.into();
}

fn compute_active(position: &Position, state: &State) -> bool {
    let grid: HashMap<Position, bool> = state.into();
    let neighbors: HashSet<Position> = compute_neighbors(position);

    let current_position_is_active = grid.get(position).unwrap_or(&false);

    let active_neighbor_count = neighbors
        .iter()
        .filter(|n| *grid.get(n).unwrap_or(&false))
        .count();

    match (current_position_is_active, active_neighbor_count) {
        (true, 2) => true,
        (_, 3) => true,
        _ => false,
    }
}

fn compute_neighbors(position: &Position) -> HashSet<Position> {
    let &(x, y, z) = position;

    let mut neighbors: HashSet<Position> = HashSet::new();

    for nx in (x - 1)..=(x + 1) {
        for ny in (y - 1)..=(y + 1) {
            for nz in (z - 1)..=(z + 1) {
                neighbors.insert((nx, ny, nz));
            }
        }
    }

    neighbors.remove(&(x, y, z));

    return neighbors;
}

fn compute_known_positions(state: &State) -> HashSet<Position> {
    let mut known_positions: HashSet<Position> = HashSet::new();

    for (position, _) in state.iter() {
        known_positions.insert(*position);
        known_positions.extend(compute_neighbors(position))
    }

    return known_positions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<(), String> {
        let state: State = "
          .#.
          ..#
          ###
        "
        .parse()?;

        let end_state = process_cycles(6, &state);
        let active_cubes = count_active_cubes(end_state);
        assert_eq!(112, active_cubes);
        Ok(())
    }
}
