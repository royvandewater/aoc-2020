use std::{cell::RefCell, collections::HashMap};

use crate::{
    rule::{Rule, RuleValue},
    rules::Rules,
};

use super::AlteredRules;

impl From<Rules> for AlteredRules {
    fn from(rules: Rules) -> Self {
        let mut rules_map: HashMap<usize, Rule> = rules.into();

        rules_map.insert(
            8,
            Rule {
                value: RuleValue::MatchOptions(vec![vec![42], vec![42, 8]]),
            },
        );

        rules_map.insert(
            11,
            Rule {
                value: RuleValue::MatchOptions(vec![vec![42, 31], vec![42, 11, 31]]),
            },
        );

        AlteredRules {
            cache: RefCell::new(HashMap::new()),
            rules: rules_map,
        }
    }
}
