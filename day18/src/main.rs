extern crate derive_error;

mod formula;
mod formula_list;

use std::fs;

use crate::formula_list::FormulaList;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
    let formula_list: FormulaList = input.parse().expect("could not parse input as FormulaList");

    let sum: isize = formula_list.iter().map(|e| e.evaluate()).sum();
    println!("Stage 2: {}", sum);
}
