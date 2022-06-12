mod instruction;
mod program;
mod repairer;

use std::fs;

use program::Program;
use repairer::Repairer;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");

    match Program::try_from(input.as_str()) {
        Err(e) => panic!("Error parsing program: {}", e),
        Ok(mut program) => {
            program.run_until_loop_detected();
            println!("Broken Accumulator: {}", program.get_accumulator());
        }
    }

    match Repairer::try_from(input.as_str()) {
        Err(e) => panic!("Error parsing program: {}", e),
        Ok(mut repairer) => match repairer.repair_and_run() {
            Err(e) => panic!("Could not repair program: {}", e),
            Ok(()) => println!("Repaired Accumulator: {}", repairer.get_accumulator()),
        },
    }
}
