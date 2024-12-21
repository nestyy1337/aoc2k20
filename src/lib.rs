use anyhow::Result;

pub fn parse_input(path: &str) -> Result<Vec<String>, anyhow::Error> {
    let mut mapped = vec![];
    let parsed = std::fs::read_to_string(path)?;
    for line in parsed.lines() {
        mapped.push(line.to_string());
    }
    Ok(mapped)
}
