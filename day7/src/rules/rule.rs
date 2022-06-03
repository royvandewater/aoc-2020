use std::collections::HashMap;

pub struct Rule {
  pub key: String,
  pub children: HashMap<String, usize>,
}

impl Rule {
  pub fn can_directly_contain(self: &Self, bag: &str) -> bool {
    self.children.contains_key(bag)
  }
}

impl<'a> From<&str> for Rule {
  fn from(input: &str) -> Rule {
    let string = String::from(input);
    let mut parts = string.split("bags contain");

    let key = parts.next().unwrap().trim();
    let children_string = parts.next().unwrap();
    let children: HashMap<String, usize> =
      children_string.split(",").map(|c| parse_child(c)).collect();

    Rule {
      key: String::from(key),
      children: children,
    }
  }
}

fn parse_child(child_str: &str) -> (String, usize) {
  let parts: Vec<&str> = child_str.trim().split(" ").collect();
  let quantity = parse_quanity(parts[0]);
  let child_parts: Vec<&str> = parts[1..parts.len() - 1].into(); // drop the leading quantity and the trailing word "bag(s)"
  let child = String::from(child_parts.join(" "));

  return (child, quantity);
}

fn parse_quanity(quantity_str: &str) -> usize {
  match quantity_str {
    qs if qs.eq("no") => 0,
    qs => usize::from_str_radix(qs, 10).unwrap(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_directly_contain_when_it_can() {
    assert_eq!(
      true,
      Rule::from("bright white bags contain 1 shiny gold bag").can_directly_contain("shiny gold")
    );
  }
}
