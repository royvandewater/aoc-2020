use std::fs;
mod accounting;

fn main() {
    let report_str = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let report = accounting::parse_input(report_str);
    let entries2 = accounting::find_entries(&report);
    let [a2, b2] = entries2;

    let entries3 = accounting::find_entries_3(&report);
    let [a3, b3, c3] = entries3;

    println!("2 entries: {}", a2 * b2);
    println!("3 entries: {}", a3 * b3 * c3);
}
