mod find_loop_size;
mod from;
mod transform;

use find_loop_size::find_loop_size;
use transform::transform;

pub struct Stage1 {
    card: usize,
    door: usize,
}

impl Stage1 {
    pub fn answer(&self) -> usize {
        let loop_size = find_loop_size(self.card);

        transform(self.door, loop_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let sut = Stage1::from([5764801, 17807724]);

        assert_eq!(14897079, sut.answer());
    }
}
