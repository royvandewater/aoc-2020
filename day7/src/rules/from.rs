use super::Rule;
use super::RuleMap;

use std::collections::HashMap;

impl From<&str> for RuleMap {
  fn from(input: &str) -> RuleMap {
    let mut rules: HashMap<String, Rule> = HashMap::new();

    for line in input.trim().lines() {
      let rule = Rule::from(line);
      let key = String::from(&rule.key);
      rules.insert(key, rule);
    }

    RuleMap { rules: rules }
  }
}
