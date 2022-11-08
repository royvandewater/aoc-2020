use std::str::FromStr;

use crate::rules::Rules;

pub struct RulesAndMessages {
    pub rules: Rules,
    pub messages: Vec<String>,
}

impl FromStr for RulesAndMessages {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (rules_input, messages_input) = input
            .trim()
            .split_once("\n\n")
            .ok_or("Could not find double line breaks to split rules & messages")?;

        Ok(RulesAndMessages {
            rules: rules_input.parse()?,
            messages: messages_input
                .lines()
                .map(|s| s.trim().to_string())
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() -> Result<(), String> {
        let sut: RulesAndMessages = "".parse()?;

        assert_eq!(0, sut.rules.len());
        assert_eq!(0, sut.messages.len());
        Ok(())
    }

    #[test]
    fn test_one_rule_two_messages() -> Result<(), String> {
        let sut: RulesAndMessages = r#"
        0: "a"

        a
        b
        "#
        .parse()?;

        assert_eq!(1, sut.rules.len());
        assert_eq!(2, sut.messages.len());
        Ok(())
    }
}
