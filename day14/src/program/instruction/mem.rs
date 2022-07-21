use std::str::FromStr;

pub struct Mem {
    pub address: u64,
    pub value: u64,
}

impl FromStr for Mem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let end_index = s
            .find("]")
            .ok_or(format!("Could not find closing ] in {}", s))?;

        let address = u64::from_str(&s[4..end_index]).map_err(|e| {
            format!(
                "failed to parse memory address: {}, '{}'",
                e,
                &s[4..end_index]
            )
        })?;

        let start_index = s.find("=").ok_or(format!("Could not find = in {}", s))? + 1;

        let rest = &s[start_index..].trim();

        let value = u64::from_str(rest).map_err(|e| format!("Could not parse u64: {}", e))?;

        Ok(Mem { address, value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_8_11() -> Result<(), String> {
        let sut: Mem = "mem[8] = 11".parse()?;

        assert_eq!(8, sut.address);
        assert_eq!(11, sut.value);
        Ok(())
    }
}
