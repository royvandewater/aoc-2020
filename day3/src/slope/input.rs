#[derive(Debug, PartialEq)]
pub enum Square {
  Unknown,
  Empty,
  Tree,
}

pub type Map = Vec<Vec<Square>>;

fn parse_char(c: char) -> Square {
  if c == '.' {
    return Square::Empty;
  }

  if c == '#' {
    return Square::Tree;
  }

  return Square::Unknown;
}

fn parse_row(row: &str) -> Vec<Square> {
  return row.chars().map(parse_char).collect();
}

pub fn parse_input(input: String) -> Map {
  let rows = input.split("\n").map(parse_row);
  return rows.collect();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn when_empty_map_count_is_0() {
    let map = parse_input(".".to_string());
    assert_eq!(Square::Empty, map[0][0])
  }
}
