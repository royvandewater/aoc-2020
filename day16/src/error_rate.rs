use crate::input::{Rule, Ticket};

pub fn error_rate(rules: &Vec<Rule>, nearby_tickets: &Vec<Ticket>) -> usize {
    nearby_tickets
        .iter()
        .map(|t| ticket_error_value(rules, t))
        .sum()
}

fn ticket_error_value(rules: &Vec<Rule>, ticket: &Ticket) -> usize {
    match ticket
        .iter()
        .find(|value| value_invalid_for_all_rules(*value, rules))
    {
        None => 0,
        Some(value) => *value,
    }
}

fn value_invalid_for_all_rules(value: &usize, rules: &Vec<Rule>) -> bool {
    rules.iter().all(|rule| !follows_rule(rule, value))
}

fn follows_rule(rule: &Rule, value: &usize) -> bool {
    rule.ranges.iter().any(|range| range.contains(value))
}
