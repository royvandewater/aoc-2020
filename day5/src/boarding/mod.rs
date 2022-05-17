use std::collections::HashSet;
use std::hash::Hash;

mod from;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Seat {
  id: u32,
}

pub fn from_input(input: &str) -> Vec<Seat> {
  input.lines().map(|l| Seat::from(l)).collect()
}

pub fn find_missing(seats: Vec<Seat>) -> Seat {
  let mut occupied_seats = HashSet::new();

  for seat in &seats {
    occupied_seats.insert(Seat { id: seat.id });
  }

  for seat in &seats {
    let next_seat = Seat { id: seat.id + 1 };
    if !occupied_seats.contains(&next_seat) {
      return next_seat;
    }
  }
  return Seat { id: 0 };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn when_from_input_given_empty_returns_empty_array() {
    let expected: Vec<Seat> = vec![];
    assert_eq!(expected, from_input(""));
  }

  #[test]
  fn when_from_input_given_first_seat() {
    let expected: Vec<Seat> = vec![Seat { id: 0 }];
    assert_eq!(expected, from_input("FFFFFFFLLL"));
  }

  #[test]
  fn when_from_input_given_last_seat() {
    let expected: Vec<Seat> = vec![Seat { id: 1023 }];
    assert_eq!(expected, from_input("BBBBBBBRRR"));
  }

  #[test]
  fn missing_seat_when_given_two() {
    assert_eq!(
      Seat { id: 1 },
      find_missing(vec![Seat { id: 0 }, Seat { id: 2 }])
    )
  }

  #[test]
  fn missing_seat_when_given_two_higher_up() {
    assert_eq!(
      Seat { id: 127 },
      find_missing(vec![Seat { id: 126 }, Seat { id: 129 }])
    )
  }
}
