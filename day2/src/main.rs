use std::fs;
mod passwords;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    let count = input
        .lines()
        .filter(|l| passwords::is_valid_password(l))
        .count();

    let type_2_count = input
        .lines()
        .filter(|l| passwords::is_valid_type_2_password(l))
        .count();

    println!("Valid Password Count: {}", count);
    println!("Valid Password Type 2 Count: {}", type_2_count);
}
