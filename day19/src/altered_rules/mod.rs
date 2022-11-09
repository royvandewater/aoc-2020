use std::{cell::RefCell, collections::HashMap};

use crate::rule::{MatchOption, Rule, RuleValue};

mod from;

pub struct AlteredRules {
    rules: HashMap<usize, Rule>,
    cache: RefCell<HashMap<(String, usize), bool>>,
}

impl AlteredRules {
    pub fn is_message_valid_for_index(&self, message: &str, index: usize) -> Result<bool, String> {
        let cache_key = (message.to_string(), index);

        if let Some(&cached_value) = self.cache.borrow().get(&cache_key) {
            return Ok(cached_value);
        }

        let value = self.is_message_valid_for_index_unmemoized(message, index)?;
        self.cache.borrow_mut().insert(cache_key, value);
        return Ok(value);
    }

    fn is_message_valid_for_index_unmemoized(
        &self,
        message: &str,
        index: usize,
    ) -> Result<bool, String> {
        let rule = self
            .rules
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
        options: &Vec<MatchOption>,
        message: &str,
    ) -> Result<bool, String> {
        for option in options {
            if self.is_option_valid_for_message(option, message)? {
                return Ok(true);
            }
        }

        return Ok(false);
    }

    fn is_option_valid_for_message(
        &self,
        indices: &MatchOption,
        message: &str,
    ) -> Result<bool, String> {
        match indices[..] {
            [i1] => self.is_1_index_valid_for_message(message, i1),
            [i1, i2] => self.are_2_indices_valid_for_message(message, i1, i2),
            [i1, i2, i3] => self.are_3_indices_valid_for_message(message, i1, i2, i3),

            _ => Err(format!(
                "Only supports 1, 2, or 3 indexes per MatchOption, received: {}",
                indices.len()
            )),
        }
    }

    fn is_1_index_valid_for_message(&self, message: &str, i1: usize) -> Result<bool, String> {
        self.is_message_valid_for_index(message, i1)
    }

    fn are_2_indices_valid_for_message(
        &self,
        message: &str,
        i1: usize,
        i2: usize,
    ) -> Result<bool, String> {
        let mut cursor = 1;

        loop {
            if cursor >= message.len() {
                return Ok(false);
            }

            let (first, second) = message.split_at(cursor);

            if self.is_message_valid_for_index(first, i1)?
                && self.is_message_valid_for_index(second, i2)?
            {
                return Ok(true);
            }

            cursor += 1;
        }
    }

    fn are_3_indices_valid_for_message(
        &self,
        message: &str,
        i1: usize,
        i2: usize,
        i3: usize,
    ) -> Result<bool, String> {
        let mut cursor_1 = 1;
        let mut cursor_2 = 2;

        loop {
            if cursor_1 >= message.len() {
                return Ok(false);
            }

            loop {
                if cursor_2 >= message.len() {
                    cursor_2 = cursor_1 + 1;
                    break;
                }

                let (chunk, third) = message.split_at(cursor_2);
                let (first, second) = chunk.split_at(cursor_1);

                if self.is_message_valid_for_index(first, i1)?
                    && self.is_message_valid_for_index(second, i2)?
                    && self.is_message_valid_for_index(third, i3)?
                {
                    return Ok(true);
                }

                cursor_2 += 1;
            }

            cursor_1 += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules::Rules;

    use super::*;

    #[test]
    fn test_rule_by_self_reference() -> Result<(), String> {
        let rules: Rules = r#"
            0: 1 3
            1: 2 | 1 2
            2: "a"
            3: "b"
        "#
        .parse()?;

        let sut: AlteredRules = rules.into();

        assert_eq!(true, sut.is_message_valid_for_index("ab", 0)?);
        assert_eq!(true, sut.is_message_valid_for_index("aab", 0)?);
        assert_eq!(true, sut.is_message_valid_for_index("aaab", 0)?);
        assert_eq!(false, sut.is_message_valid_for_index("b", 0)?);
        assert_eq!(false, sut.is_message_valid_for_index("bab", 0)?);

        Ok(())
    }

    #[test]
    fn test_example_1() -> Result<(), String> {
        let rules: Rules = r#"
            42: 9 14 | 10 1
            9: 14 27 | 1 26
            10: 23 14 | 28 1
            1: "a"
            11: 42 31
            5: 1 14 | 15 1
            19: 14 1 | 14 14
            12: 24 14 | 19 1
            16: 15 1 | 14 14
            31: 14 17 | 1 13
            6: 14 14 | 1 14
            2: 1 24 | 14 4
            0: 8 11
            13: 14 3 | 1 12
            15: 1 | 14
            17: 14 2 | 1 7
            23: 25 1 | 22 14
            28: 16 1
            4: 1 1
            20: 14 14 | 1 15
            3: 5 14 | 16 1
            27: 1 6 | 14 18
            14: "b"
            21: 14 1 | 1 14
            25: 1 1 | 1 14
            22: 14 14
            8: 42
            26: 14 22 | 1 20
            18: 15 15
            7: 14 5 | 1 21
            24: 14 1
        "#
        .parse()?;

        let sut: AlteredRules = rules.into();

        assert_eq!(true, sut.is_message_valid_for_index("bbabbbbaabaabba", 0)?);
        assert_eq!(
            false,
            sut.is_message_valid_for_index("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa", 0)?
        );
        assert_eq!(
            true,
            sut.is_message_valid_for_index("aaabbbbbbaaaabaababaabababbabaaabbababababaaa", 0)?
        );

        Ok(())
    }
}
