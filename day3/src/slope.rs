mod input;

pub fn parse_input(string: String) -> input::Map {
  return input::parse_input(string);
}

fn square_for_position(map: &input::Map, x: usize, y: usize) -> Option<input::Square> {
  let row = map.get(y)?;
  let square = row.get(x % row.len())?;

  return match square {
    input::Square::Empty => Some(input::Square::Empty),
    input::Square::Tree => Some(input::Square::Tree),
    input::Square::Unknown => Some(input::Square::Unknown),
  };
}

fn value_for_position(map: &input::Map, x: usize, y: usize) -> usize {
  let square = square_for_position(map, x, y);

  return match square.unwrap_or(input::Square::Empty) {
    input::Square::Empty => 0,
    input::Square::Tree => 1,
    input::Square::Unknown => panic!("Received unknown tile"),
  };
}

pub fn count_trees_hit(map: &input::Map, dx: usize, dy: usize) -> usize {
  let mut px = 0;
  let mut py = 0;

  let mut count = 0;

  let map_height = map.len();

  while py < map_height {
    count += value_for_position(&map, px, py);
    px += dx;
    py += dy;
  }

  return count;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn when_empty_map_count_is_0() {
    let map = parse_input(".".to_string());
    assert_eq!(0, count_trees_hit(&map, 1, 1));
  }

  #[test]
  fn when_map_has_only_a_tree() {
    let map = parse_input("#".to_string());
    assert_eq!(1, count_trees_hit(&map, 1, 1));
  }

  #[test]
  fn example_1() {
    let map_string = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
"
    .trim()
    .to_string();

    let map = parse_input(map_string);
    assert_eq!(7, count_trees_hit(&map, 3, 1))
  }
}
