use std::{collections::HashMap, str::FromStr};

use crate::rule::Rule;

use super::Rules;

impl FromStr for Rules {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let rules: HashMap<usize, Rule> = input
            .trim()
            .lines()
            .map(|line| {
                let (index_str, text) = line
                    .trim()
                    .split_once(":")
                    .ok_or(format!("Failed to split rule string on ':': '{}'", line))?;

                let index = usize::from_str(index_str.trim())
                    .map_err(|e| format!("Failed ot parse a rule index as usize: {}", e))?;

                let rule: Rule = text.parse()?;

                Ok((index, rule))
            })
            .collect::<Result<HashMap<usize, Rule>, String>>()?;

        Ok(Rules(rules))
    }
}
