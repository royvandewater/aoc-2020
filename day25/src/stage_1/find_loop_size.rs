const SUBJECT_NUMBER: usize = 7;
const DIVIDENT: usize = 20201227;

pub fn find_loop_size(card: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;

    while value != card {
        value = (value * SUBJECT_NUMBER) % DIVIDENT;
        loop_size += 1;
    }

    loop_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1_card() {
        assert_eq!(8, find_loop_size(5764801));
    }

    #[test]
    fn test_example_1_door() {
        assert_eq!(11, find_loop_size(17807724));
    }
}
