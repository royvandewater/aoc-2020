mod rules;

use std::fs;

use rules::RuleMap;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let rule_map = RuleMap::from(input.as_str());

    println!(
        "Bags that can contain Shiny Gold: {:?}",
        rule_map.count_can_contain("shiny gold")
    );
    println!(
        "Shiny Gold contains: {:?}",
        rule_map.count_contained_bags("shiny gold")
    );
}
