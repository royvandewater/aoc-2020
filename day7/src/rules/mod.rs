mod from;
mod rule;

use rule::Rule;
use std::collections::HashMap;

pub struct RuleMap {
  rules: HashMap<String, Rule>,
}

impl RuleMap {
  pub fn count_can_contain(self: &Self, bag: &str) -> usize {
    self
      .rules
      .keys()
      .cloned()
      .filter(|k| can_contain(&self.rules, k, bag))
      .count()
  }

  pub fn count_contained_bags(self: &Self, bag: &str) -> usize {
    if bag.eq("other") {
      return 0;
    }

    let rule = self.rules.get(bag).unwrap();

    let mut total = 0;

    for (child, quantity) in &rule.children {
      total += quantity;
      total += quantity * self.count_contained_bags(child.as_str());
    }

    return total;
  }
}

fn can_contain(rule_map: &HashMap<String, Rule>, rule_key: &str, bag: &str) -> bool {
  match rule_map.get(rule_key) {
    None => return false,
    Some(r) => {
      if r.can_directly_contain(bag) {
        return true;
      }

      return r
        .children
        .iter()
        .any(|(c, _q)| can_contain(rule_map, c, bag));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn count_when_given_empty_ruleset() {
    assert_eq!(0, RuleMap::from("").count_can_contain("shiny gold"));
  }

  #[test]
  fn count_when_given_one_rule() {
    let rule_strs = "
      bright white bags contain 1 shiny gold bag.
    ";

    assert_eq!(1, RuleMap::from(rule_strs).count_can_contain("shiny gold"));
  }

  #[test]
  fn count_when_bag_is_target_but_cant_contain_it() {
    let rule_strs = "
      shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    ";

    assert_eq!(0, RuleMap::from(rule_strs).count_can_contain("shiny gold"));
  }

  #[test]
  fn count_when_given_two_nested_rules() {
    let rule_strs = "
      bright white bags contain 1 shiny gold bag.
      shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
    ";

    assert_eq!(2, RuleMap::from(rule_strs).count_can_contain("shiny gold"));
  }

  #[test]
  fn count_when_given_one_irrelevant_rule() {
    let rule_strs = "
      shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
      faded blue bags contain no other bags.
    ";

    assert_eq!(1, RuleMap::from(rule_strs).count_can_contain("shiny gold"));
  }

  #[test]
  fn count_example_1() {
    let rule_strs = "
      light red bags contain 1 bright white bag, 2 muted yellow bags.
      dark orange bags contain 3 bright white bags, 4 muted yellow bags.
      bright white bags contain 1 shiny gold bag.
      muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
      shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
      dark olive bags contain 3 faded blue bags, 4 dotted black bags.
      vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
      faded blue bags contain no other bags.
      dotted black bags contain no other bags.
    ";

    assert_eq!(4, RuleMap::from(rule_strs).count_can_contain("shiny gold"));
  }

  #[test]
  fn count_contained_bags_none() {
    let rule_strs = "
      shiny gold bags contain no other bags.
    ";

    assert_eq!(
      0,
      RuleMap::from(rule_strs).count_contained_bags("shiny gold"),
    );
  }

  #[test]
  fn count_contained_bags_one_no_nesting() {
    let rule_strs = "
      shiny gold bags contain 1 dark olive bag.
      dark olive bags contain no other bags.
    ";

    assert_eq!(
      1,
      RuleMap::from(rule_strs).count_contained_bags("shiny gold"),
    );
  }

  #[test]
  fn count_contained_bags_example_1() {
    let rule_strs = "
      light red bags contain 1 bright white bag, 2 muted yellow bags.
      dark orange bags contain 3 bright white bags, 4 muted yellow bags.
      bright white bags contain 1 shiny gold bag.
      muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
      shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
      dark olive bags contain 3 faded blue bags, 4 dotted black bags.
      vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
      faded blue bags contain no other bags.
      dotted black bags contain no other bags.
    ";

    assert_eq!(
      32,
      RuleMap::from(rule_strs).count_contained_bags("shiny gold"),
    );
  }

  #[test]
  fn count_contained_bags_example_2() {
    let rule_strs = "
      shiny gold bags contain 2 dark red bags.
      dark red bags contain 2 dark orange bags.
      dark orange bags contain 2 dark yellow bags.
      dark yellow bags contain 2 dark green bags.
      dark green bags contain 2 dark blue bags.
      dark blue bags contain 2 dark violet bags.
      dark violet bags contain no other bags.
    ";

    assert_eq!(
      126,
      RuleMap::from(rule_strs).count_contained_bags("shiny gold"),
    );
  }
}
