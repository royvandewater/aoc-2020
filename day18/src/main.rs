extern crate derive_error;

mod expression;
mod expression_list;

use std::fs;

use crate::expression_list::ExpressionList;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("could not read ./input.txt");
    let expression_list: ExpressionList = input
        .parse()
        .expect("could not parse input as ExpressionList");

    let sum: isize = expression_list.iter().map(|e| e.evaluate()).sum();
    println!("Stage 1: {}", sum);
}
