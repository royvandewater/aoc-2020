use std::collections::HashMap;

pub fn round(
    max: usize,
    current: usize,
    mut cups: HashMap<usize, usize>,
) -> (usize, HashMap<usize, usize>) {
    let &first = cups.get(&current).unwrap();
    let &second = cups.get(&first).unwrap();
    let &third = cups.get(&second).unwrap();

    let picked_up = vec![first, second, third];
    let destination = get_destination(max, current, &picked_up);
    let &after_destination = cups.get(&destination).unwrap();

    cups.insert(destination, first);
    cups.insert(third, after_destination);

    let &next = cups.get(&current).unwrap();

    return (next, cups);
}

fn get_destination(max: usize, current: usize, picked_up: &Vec<usize>) -> usize {
    let mut destination = decrement(max, current);

    while picked_up.contains(&destination) {
        destination = decrement(max, destination);
    }

    destination
}

fn decrement(max: usize, current: usize) -> usize {
    match current {
        1 => max,
        _ => current - 1,
    }
}
