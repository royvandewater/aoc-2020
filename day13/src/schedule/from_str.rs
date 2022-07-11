use std::{num::ParseIntError, str::FromStr};

use super::Schedule;

impl FromStr for Schedule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let start_time: usize = lines
            .next()
            .ok_or(format!("Could not find start time in: '{}'", s))?
            .trim()
            .parse()
            .map_err(|e| format!("Failed to parse start_time: {}", e))?;

        let intervals: Vec<usize> = lines
            .next()
            .ok_or(format!("Could not find intervals in: '{}'", s))?
            .trim()
            .split(',')
            .filter(|s| *s != "x")
            .map(usize::from_str)
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .map_err(|e| format!("Failed to parse interval as usize: {}", e))?;

        Ok(Schedule {
            start_time,
            intervals,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_time_0() -> Result<(), String> {
        let sut: Schedule = "
          0
          7
        "
        .parse()?;

        assert_eq!(0, sut.start_time);
        Ok(())
    }

    #[test]
    fn start_time_923() -> Result<(), String> {
        let sut: Schedule = "
          923
          7
        "
        .parse()?;

        assert_eq!(923, sut.start_time);
        Ok(())
    }

    #[test]
    fn interval_7() -> Result<(), String> {
        let sut: Schedule = "
          0
          7
        "
        .parse()?;

        assert_eq!(vec![7], sut.intervals);
        Ok(())
    }

    #[test]
    fn interval_7_10_15() -> Result<(), String> {
        let sut: Schedule = "
          0
          7,10,15
        "
        .parse()?;

        assert_eq!(vec![7, 10, 15], sut.intervals);
        Ok(())
    }

    #[test]
    fn interval_7_x_15() -> Result<(), String> {
        let sut: Schedule = "
          0
          7,x,15
        "
        .parse()?;

        assert_eq!(vec![7, 15], sut.intervals);
        Ok(())
    }
}
