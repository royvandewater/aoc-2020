use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Weakness {
    pub min: usize,
    pub max: usize,
}

impl fmt::Display for Weakness {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {} = {}", self.min, self.max, self.min + self.max)
    }
}
