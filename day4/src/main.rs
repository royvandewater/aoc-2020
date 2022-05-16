#[macro_use]
extern crate lazy_static;

use std::fs;
mod validate_passport;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    let passport_strings = validate_passport::collect_passport_strings(input.as_str());

    let valid_count = passport_strings
        .into_iter()
        .filter(|p| validate_passport::valid_passport_string(p))
        .count();

    println!("Valid Passports: {}", valid_count);
}
