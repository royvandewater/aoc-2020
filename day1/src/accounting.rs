pub fn find_entries(expenses: &[i32]) -> [i32; 2] {
  for expense_1 in expenses {
    for expense_2 in expenses {
      if expense_1 == expense_2 {
        continue;
      }

      if expense_1 + expense_2 == 2020 {
        return [*expense_1, *expense_2];
      }
    }
  }

  return [0, 0];
}

pub fn find_entries_3(expenses: &[i32]) -> [i32; 3] {
  for expense_1 in expenses {
    for expense_2 in expenses {
      if expense_1 == expense_2 {
        continue;
      }

      for expense_3 in expenses {
        if expense_1 == expense_3 || expense_2 == expense_3 {
          continue;
        }

        if expense_1 + expense_2 + expense_3 == 2020 {
          return [*expense_1, *expense_2, *expense_3];
        }
      }
    }
  }

  return [0, 0, 0];
}

pub fn parse_input<'a>(input: String) -> Vec<i32> {
  let lines = input
    .lines()
    .filter_map(|l| l.to_string().parse::<i32>().ok());
  return lines.collect();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_entries_example_1() {
    let report = &[1721, 979, 366, 299, 675, 1456];
    let entries = find_entries(report);
    assert_eq!(entries, [1721, 299]);
  }

  #[test]
  fn find_entries_3_example_1() {
    let report = &[1721, 979, 366, 299, 675, 1456];
    let entries = find_entries_3(report);
    assert_eq!(entries, [979, 366, 675]);
  }

  #[test]
  fn parse_input_test() {
    let report = parse_input(
      "
1721
979
366
299
675
1456
    "
      .into(),
    );
    assert_eq!(report, &[1721, 979, 366, 299, 675, 1456]);
  }
}
