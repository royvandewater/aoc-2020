use std::str::FromStr;

use super::location::Location;
use super::{Grid, Plane2};

pub fn parse_grid(s: &str) -> Result<Grid, String> {
    let mut grid: Vec<Vec<Location>> = Vec::with_capacity(s.trim().lines().count());

    for line in s.trim().lines() {
        grid.push(
            line.trim()
                .chars()
                .map(|c| Location::try_from(c))
                .collect::<Result<Vec<Location>, String>>()?,
        );
    }

    Ok(grid)
}

impl FromStr for Plane2 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Plane2 {
            grid: parse_grid(s)?,
        })
    }
}
