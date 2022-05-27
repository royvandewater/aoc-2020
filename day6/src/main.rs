mod groups;

use groups::Group;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let groups: Vec<Group> = groups::from_input(&input);

    let sum_groups = groups::sum_groups(groups.iter().copied().collect());
    let sum_groups_alt = groups::sum_groups_alt(groups.iter().copied().collect());

    println!("Sum: {:?}", sum_groups);
    println!("Sum Alt: {:?}", sum_groups_alt);
}
