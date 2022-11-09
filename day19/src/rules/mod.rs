use std::collections::{HashMap, HashSet};

mod from_str;
mod into;

use crate::rule::{Rule, RuleValue};

pub struct Rules(HashMap<usize, Rule>);

impl Rules {
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        0
    }

    pub fn is_message_valid_for_index(&self, message: &str, index: usize) -> Result<bool, String> {
        let rule = self
            .0
            .get(&index)
            .ok_or(format!("Failed to find rule with index: {}", index))?;

        match rule.value {
            RuleValue::String(ref s) => Ok(s == message),
            RuleValue::MatchOptions(ref options) => {
                self.are_any_options_valid_for_message(options, message)
            }
        }
    }

    fn are_any_options_valid_for_message(
        &self,
        options: &Vec<Vec<usize>>,
        message: &str,
    ) -> Result<bool, String> {
        for option in options {
            if self.are_all_index_valid_for_message(&option, message)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn are_all_index_valid_for_message(
        &self,
        indices: &Vec<usize>,
        message: &str,
    ) -> Result<bool, String> {
        let mut cursor = 0;

        for index in indices {
            let size: usize = self.size_of_rule(index)?;
            let part = &message[cursor..cursor + size];
            cursor = cursor + size;

            if !self.is_message_valid_for_index(part, *index)? {
                return Ok(false);
            }
        }

        if cursor != message.len() {
            return Ok(false); // The end of the message wasn't covered by any rule
        }

        return Ok(true);
    }

    fn size_of_rule(&self, index: &usize) -> Result<usize, String> {
        let rule = self
            .0
            .get(index)
            .ok_or(format!("Could not find rule: {}", index))?;

        match rule.value {
            RuleValue::String(ref s) => Ok(s.len()),
            RuleValue::MatchOptions(ref options) => {
                let sizes: HashSet<usize> = options
                    .iter()
                    .map(|options| options.iter().map(|option| self.size_of_rule(option)).sum())
                    .collect::<Result<HashSet<usize>, String>>()?;

                if sizes.len() > 1 {
                    return Err(format!(
                        "Expected only one rule size, received: {:?}",
                        sizes
                    ));
                }

                Ok(sizes.iter().sum())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplest_rule() -> Result<(), String> {
        let sut: Rules = r#"
            0: "a"
        "#
        .parse()?;

        assert_eq!(true, sut.is_message_valid_for_index("a", 0)?);
        assert_eq!(false, sut.is_message_valid_for_index("b", 0)?);
        Ok(())
    }

    #[test]
    fn test_rule_by_reference() -> Result<(), String> {
        let sut: Rules = r#"
            0: 1
            1: "b"
        "#
        .parse()?;

        assert_eq!(true, sut.is_message_valid_for_index("b", 0)?);
        assert_eq!(false, sut.is_message_valid_for_index("a", 0)?);
        Ok(())
    }

    #[test]
    fn test_example_1_valid() -> Result<(), String> {
        let sut: Rules = r#"
            0: 1 2
            1: "a"
            2: 1 3 | 3 1
            3: "b"
        "#
        .parse()?;

        assert_eq!(true, sut.is_message_valid_for_index("aab", 0)?);
        Ok(())
    }

    #[test]
    fn test_example_2_valid() -> Result<(), String> {
        let sut: Rules = r#"
            0: 4 1 5
            1: 2 3 | 3 2
            2: 4 4 | 5 5
            3: 4 5 | 5 4
            4: "a"
            5: "b"
        "#
        .parse()?;

        assert_eq!(true, sut.is_message_valid_for_index("ababbb", 0)?);
        assert_eq!(false, sut.is_message_valid_for_index("bababa", 0)?);
        assert_eq!(true, sut.is_message_valid_for_index("abbbab", 0)?);
        assert_eq!(false, sut.is_message_valid_for_index("aaabbb", 0)?);
        assert_eq!(false, sut.is_message_valid_for_index("aaaabbb", 0)?);
        Ok(())
    }
}
