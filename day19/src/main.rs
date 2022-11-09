extern crate derive_error;

mod altered_rules;
mod rule;
mod rules;
mod rules_and_messages;

use std::fs;

use crate::{altered_rules::AlteredRules, rules_and_messages::RulesAndMessages};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
    let rules_and_messages: RulesAndMessages = input
        .parse()
        .expect("could not parse input as RulesAndMessages");

    let messages = rules_and_messages.messages;
    let rules = rules_and_messages.rules;

    let potential_values_count: usize = messages
        .iter()
        .filter(|message| rules.is_message_valid_for_index(message, 0).unwrap())
        .count();

    println!("Stage 1: {}", potential_values_count);

    let altered_rules: AlteredRules = rules.into();
    let potential_values_count: usize = messages
        .iter()
        .filter(|message| {
            altered_rules
                .is_message_valid_for_index(message, 0)
                .unwrap()
        })
        .count();

    println!("Stage 2: {}", potential_values_count);
}
