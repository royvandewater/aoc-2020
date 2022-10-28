extern crate derive_error;

mod error_rate;
mod input;

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use error_rate::{error_rate, follows_rule};
use input::{Input, Rule, Ticket};

use crate::error_rate::ticket_is_valid;

fn main() {
    let input: Input = fs::read_to_string("./input.txt")
        .expect("could not read ./input.txt")
        .parse()
        .expect("could not parse input as Input");

    println!(
        "Stage 1: {}",
        error_rate(&input.rules, &input.nearby_tickets)
    );
    let product = find_product(input);

    println!("Stage 2: {}", product);
}

fn find_product(input: Input) -> usize {
    let rules = get_assigned_rules(input.clone());

    rules
        .iter()
        .filter(|rule| rule.label.starts_with("departure"))
        .map(|rule| {
            *input
                .your_ticket
                .iter()
                .nth(rule.position.unwrap())
                .unwrap()
        })
        .product()
}

fn get_assigned_rules(input: Input) -> Vec<Rule> {
    let mut all_tickets = Vec::from([input.your_ticket.clone()]);
    all_tickets.append(&mut input.nearby_tickets.clone());
    let valid_tickets: Vec<Ticket> = all_tickets
        .iter()
        .filter(|ticket| ticket_is_valid(&input.rules, ticket))
        .cloned()
        .collect();

    let mut rules: Vec<Rule> = input.rules.iter().cloned().collect();
    assign_rules(&mut rules, &valid_tickets);
    return rules;
}

fn assign_rules(rules: &mut Vec<Rule>, tickets: &Vec<Ticket>) {
    let mut rules_by_label: HashMap<String, &mut Rule> = HashMap::new();

    for rule in rules {
        rules_by_label.insert(rule.label.clone(), rule);
    }

    let rule_labels: HashSet<String> = rules_by_label.keys().cloned().collect();

    let mut unassigned_positions: HashMap<usize, HashSet<String>> = rules_by_label
        .iter()
        .enumerate()
        .map(|(i, _)| (i, rule_labels.clone()))
        .collect();

    let mut last_count = 0;
    loop {
        let count = unassigned_positions.len();
        if last_count == count {
            panic!(
                "We made no progress last iteration! {} == {}",
                last_count, count
            );
        }
        last_count = count;

        if unassigned_positions.is_empty() {
            return;
        }

        for (position, potential_rule_labels) in unassigned_positions.iter_mut() {
            potential_rule_labels.retain(|rule_label| {
                let rule = rules_by_label.get(rule_label).unwrap();

                tickets
                    .iter()
                    .all(|ticket| follows_rule(rule, &ticket[*position]))
            })
        }

        let (position, potential_rule_labels) = unassigned_positions
            .iter_mut()
            .find(|(_, labels)| labels.len() == 1)
            .ok_or("Couldn't find any rules with length one, which will result in an infinite loop")
            .unwrap();

        let label = potential_rule_labels.iter().next().unwrap();
        let mut rule = rules_by_label.get_mut(label).unwrap();
        rule.position = Some(*position);

        let label = label.clone();
        unassigned_positions.remove(&rule.position.unwrap());
        unassigned_positions.iter_mut().for_each(|(_, v)| {
            v.remove(&label);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<(), String> {
        let input: Input = "
        class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
        "
        .parse()?;

        let rules = get_assigned_rules(input.clone());
        let ticket = input.your_ticket.clone();

        assert_eq!(3, rules.len());
        let rule_class = rules
            .iter()
            .find(|r| r.label == "class")
            .ok_or("Couldn't find rule class")?;

        assert_eq!(12, *ticket.get(rule_class.position.unwrap()).unwrap());

        let rule_row = rules
            .iter()
            .find(|r| r.label == "row")
            .ok_or("Couldn't find rule row")?;

        assert_eq!(11, *ticket.get(rule_row.position.unwrap()).unwrap());

        let rule_seat = rules
            .iter()
            .find(|r| r.label == "seat")
            .ok_or("Couldn't find rule seat")?;

        assert_eq!(13, *ticket.get(rule_seat.position.unwrap()).unwrap());

        Ok(())
    }
}
