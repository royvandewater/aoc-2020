use std::str::FromStr;

use super::Waterfall;

impl FromStr for Waterfall {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        lines.next(); // discard the start_time

        let intervals: Vec<usize> = lines
            .next()
            .ok_or(format!("Could not find intervals in: '{}'", s))?
            .trim()
            .split(',')
            .map(|x| usize::from_str(x).unwrap_or(1))
            .collect();

        Ok(Waterfall { intervals })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_7() -> Result<(), String> {
        let sut: Waterfall = "
          0
          7
        "
        .parse()?;

        assert_eq!(vec![7], sut.intervals);
        Ok(())
    }

    #[test]
    fn interval_7_10_15() -> Result<(), String> {
        let sut: Waterfall = "
          0
          7,10,15
        "
        .parse()?;

        assert_eq!(vec![7, 10, 15], sut.intervals);
        Ok(())
    }

    #[test]
    fn interval_7_x_15() -> Result<(), String> {
        let sut: Waterfall = "
          0
          7,x,15
        "
        .parse()?;

        assert_eq!(vec![7, 1, 15], sut.intervals);
        Ok(())
    }
}
