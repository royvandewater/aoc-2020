use std::str::FromStr;

pub struct Item {
    pub id: usize,
    pub tile: Vec<String>,
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();

        let id: usize = lines
            .next()
            .ok_or("Could not fetch header line")?
            .replace("Tile", "")
            .replace(":", "")
            .trim()
            .parse()
            .map_err(|e| format!("Failed to parse id: {}", e))?;

        let tile: Vec<String> = lines.map(|l| l.trim().to_string()).collect();

        Ok(Item { id, tile })
    }
}
