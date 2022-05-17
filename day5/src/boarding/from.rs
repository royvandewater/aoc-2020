use super::Seat;

impl From<&str> for Seat {
  fn from(input: &str) -> Seat {
    let row = parse_row(input);
    let col = parse_col(input);

    Seat {
      id: (row * 8) + col,
    }
  }
}

fn parse_row(input: &str) -> u32 {
  let binary_str = &input[0..=6].replace("F", "0").replace("B", "1");
  u32::from_str_radix(binary_str, 2).unwrap_or(0)
}

fn parse_col(input: &str) -> u32 {
  let binary_str = &input[7..=9].replace("L", "0").replace("R", "1");
  u32::from_str_radix(binary_str, 2).unwrap_or(0)
}

#[cfg(test)]
mod tests {
  use crate::boarding::Seat;

  #[test]
  fn when_from_input_given_first_seat() {
    assert_eq!(Seat { id: 0 }, Seat::from("FFFFFFFLLL"));
  }

  #[test]
  fn when_from_input_given_last_seat() {
    assert_eq!(Seat { id: 1023 }, Seat::from("BBBBBBBRRR"));
  }
}
