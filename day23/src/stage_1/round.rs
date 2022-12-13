use std::collections::VecDeque;

pub fn round(mut cups: VecDeque<usize>) -> VecDeque<usize> {
    let current = cups.pop_front().unwrap();
    let picked_up = (0..3).map(|_| cups.pop_front().unwrap()).collect();
    let destination = get_destination(current, &picked_up);
    let index = cups
        .iter()
        .enumerate()
        .find_map(|(i, &cup)| match cup == destination {
            true => Some(1 + (i % 9)),
            false => None,
        })
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
        1 => 9,
        _ => current - 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_example_1_round_1() {
        let result = round(VecDeque::from([3, 8, 9, 1, 2, 5, 4, 6, 7]));

        assert_eq!(vec![2, 8, 9, 1, 5, 4, 6, 7, 3], Vec::from(result))
    }
}
