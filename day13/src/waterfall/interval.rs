use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Interval {
    X,
    Time(usize),
}

impl FromStr for Interval {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Interval::X),
            _ => match usize::from_str(s) {
                Ok(i) => Ok(Interval::Time(i)),
                Err(e) => Err(format!("Could not parse time: {}, {}", s, e)),
            },
        }
    }
}
