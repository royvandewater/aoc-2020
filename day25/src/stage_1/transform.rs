const DIVIDENT: usize = 20201227;

pub fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;

    for _ in 0..loop_size {
        value = (value * subject_number) % DIVIDENT;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_card() {
        assert_eq!(14897079, transform(17807724, 8));
    }

    #[test]
    fn test_example_1_door() {
        assert_eq!(14897079, transform(5764801, 11));
    }
}
