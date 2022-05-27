mod from;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Group {
  pub value: usize,
  pub value_alt: usize,
}

pub fn from_input(input: &str) -> Vec<Group> {
  let group_strs: Vec<&str> = input.split("\n\n").collect();
  group_strs.iter().map(|s| Group::from(*s)).collect()
}

pub fn sum_groups(groups: Vec<Group>) -> usize {
  let values: Vec<usize> = groups.iter().map(|g| g.value).collect();
  let sum: usize = values.iter().sum();
  return sum;
}

pub fn sum_groups_alt(groups: Vec<Group>) -> usize {
  let values: Vec<usize> = groups.iter().map(|g| g.value_alt).collect();
  let sum: usize = values.iter().sum();
  return sum;
}
