use std::str::FromStr;

pub type MatchOption = Vec<usize>;

pub enum RuleValue {
    String(String),
    MatchOptions(Vec<MatchOption>),
}

pub struct Rule {
    pub value: RuleValue,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        match s.matches("\"").count() {
            1 => Err(format!("Found only one quote in rule text: '{}'", s)),
            2 => Ok(Rule {
                value: RuleValue::String(s.replace("\"", "").to_string()),
            }),
            _ => Ok(Rule {
                value: parse_match_options(s)?,
            }),
        }
    }
}

fn parse_match_options(value: &str) -> Result<RuleValue, String> {
    let pairs = value
        .split("|")
        .map(|s| parse_match_option(s))
        .collect::<Result<Vec<MatchOption>, String>>()?;

    Ok(RuleValue::MatchOptions(pairs))
}

fn parse_match_option(value: &str) -> Result<MatchOption, String> {
    let references: MatchOption = value
        .trim()
        .split(" ")
        .map(|s| {
            usize::from_str(s).map_err(|e| format!("Failed to parse reference: '{}'. {}", s, e))
        })
        .collect::<Result<MatchOption, String>>()?;

    Ok(references)
}
