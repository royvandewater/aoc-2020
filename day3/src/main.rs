use std::fs;
mod slope;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read input.txt");
    let map = slope::parse_input(input);

    let slopes: Vec<[usize; 2]> = vec![[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

    let total = slopes
        .iter()
        .map(|[dx, dy]| slope::count_trees_hit(&map, *dx, *dy))
        .reduce(|acc, count| acc * count)
        .unwrap_or(0);

    println!("Trees Hit: {}", total);
}
