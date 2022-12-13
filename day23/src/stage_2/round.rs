use std::collections::VecDeque;

const ONE_MILLION: usize = 1000 * 1000;

pub fn round(mut cups: VecDeque<usize>) -> VecDeque<usize> {
    let current = cups.pop_front().unwrap();
    let picked_up = (0..3).map(|_| cups.pop_front().unwrap()).collect();
    let destination = get_destination(current, &picked_up);
    let index = cups
        .iter()
        .enumerate()
        .find_map(|(i, &cup)| match cup == destination {
            true => Some(1 + (i % ONE_MILLION)),
            false => None,
        })
        .ok_or(format!(
            "Failed to find index of: {}. current: {}, picked_up: {:?}\n{:?}",
            destination, current, picked_up, cups
        ))
        .unwrap();

    for (i, &cup) in picked_up.iter().enumerate() {
        cups.insert(index + i, cup);
    }

    cups.push_back(current);

    return cups;
}

fn get_destination(current: usize, picked_up: &VecDeque<usize>) -> usize {
    let mut destination = decrement(current);

    while picked_up.contains(&destination) {
        destination = decrement(destination);
    }

    destination
}

fn decrement(current: usize) -> usize {
    match current {
        1 => ONE_MILLION,
        _ => current - 1,
    }
}
