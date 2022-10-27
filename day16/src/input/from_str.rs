use std::{num::ParseIntError, ops::Range, str::FromStr};

use super::{Input, Rule, Ticket};

impl FromStr for Input {
    type Err = String;

    fn from_str(stream: &str) -> Result<Self, Self::Err> {
        let mut parts = stream.trim().splitn(3, "\n\n");

        let rules = parts
            .next()
            .ok_or("Failed to find rules section")?
            .lines()
            .map(parse_rule)
            .collect::<Result<Vec<Rule>, String>>()?;

        let your_ticket = parts
            .next()
            .ok_or("Failed to find your_ticket section")?
            .lines()
            .skip(1)
            .map(parse_ticket)
            .collect::<Result<Vec<Ticket>, String>>()?
            .first()
            .ok_or("Could not find your ticket")?
            .clone();

        let nearby_tickets = parts
            .next()
            .ok_or("Failed to find nearby_tickets section")?
            .lines()
            .skip(1)
            .map(parse_ticket)
            .collect::<Result<Vec<Ticket>, String>>()?;

        Ok(Input {
            rules,
            your_ticket,
            nearby_tickets,
        })
    }
}

fn parse_rule(input: &str) -> Result<Rule, String> {
    let (label, ranges_str) = input
        .trim()
        .split_once(": ")
        .ok_or(format!("Failed to split rule on ':': '{}'", input))?;

    let ranges = ranges_str
        .trim()
        .split(" or ")
        .map(parse_range)
        .collect::<Result<Vec<Range<usize>>, String>>()?;

    Ok(Rule {
        label: label.into(),
        ranges,
    })
}

fn parse_range(input: &str) -> Result<Range<usize>, String> {
    let (start_str, end_str) = input
        .trim()
        .split_once("-")
        .ok_or(format!("Failed to split range on -: '{}'", input))?;

    let start =
        usize::from_str(start_str).map_err(|e| format!("Failed to parse start of range: {}", e))?;
    let end =
        usize::from_str(end_str).map_err(|e| format!("Failed to parse end of range: {}", e))?;

    Ok(Range { start, end })
}

fn parse_ticket(input: &str) -> Result<Ticket, String> {
    input
        .trim()
        .split(",")
        .map(usize::from_str)
        .collect::<Result<Ticket, ParseIntError>>()
        .map_err(|e| format!("Failed to parse ticket value: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_rule_one_nearby_ticket() -> Result<(), String> {
        let sut: Input = "
        rule a: 1-2 or 4-5

        your ticket:
        1

        nearby tickets:
        5"
        .parse()?;

        assert_eq!(1, sut.rules.len());
        assert_eq!(1, sut.your_ticket.len());
        assert_eq!(1, sut.nearby_tickets.len());
        Ok(())
    }
}
