use std::fs;

mod schedule;
use schedule::Schedule;

mod waterfall;
use waterfall::Waterfall;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let schedule: Schedule = input.parse().expect("Could not parse Schedule");
    let waterfall: Waterfall = input.parse().expect("Could not parse Waterfall");

    println!("Earliest Bus: {}", schedule.answer());
    println!("Waterfall start: {}", waterfall.start_time());
}
