use anyhow::{Context, Result};
use serde::de::{self, Error};
use serde::{Deserialize, Deserializer};

// wanted to go with serde, but realized its wayy to much work writing
// deserialize impl for all those fields with custom validation
#[derive(Debug, Deserialize)]
pub struct Passport {
    #[serde(deserialize_with = "validate_ecl")]
    ecl: String,

    #[serde(deserialize_with = "validate_pid")]
    pid: String,

    #[serde(deserialize_with = "validate_eyr")]
    eyr: usize,

    #[serde(deserialize_with = "validate_hcl")]
    hcl: String,

    #[serde(deserialize_with = "validate_byr")]
    byr: usize,

    #[serde(deserialize_with = "validate_iyr")]
    iyr: usize,

    #[serde(default)] // Optional field with no validation
    cid: Option<i32>,

    #[serde(deserialize_with = "validate_hgt")]
    hgt: String,
}

fn validate_hgt<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let hgt = String::deserialize(deserializer)?;
    if hgt.len() < 4 {
        return Err(D::Error::custom(format!(
            "Invalid lenght of field hgt. Required at least 4, provided: {:?}",
            hgt.len()
        )));
    }
    let metric = &hgt[hgt.len() - 2..];
    let height = &hgt[..hgt.len() - 2]
        .parse::<i32>()
        .with_context(|| format!("Failed to parse height to i32"))
        .map_err(de::Error::custom)?;

    match metric {
        "cm" if (150..=193).contains(&*height) => Ok(hgt),
        "in" if (59..=76).contains(&*height) => Ok(hgt),
        "cm" => Err(de::Error::custom(format!(
            "Height in cm out of range: {}",
            height
        ))),
        "in" => Err(de::Error::custom(format!(
            "Height in in out of range: {}",
            height
        ))),
        _ => Err(de::Error::custom(format!(
            "Invalid metric in hgt: {}",
            metric
        ))),
    }
}

fn validate_iyr<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let year = usize::deserialize(deserializer)?;

    if year >= 2010 && year <= 2020 {
        return Ok(year);
    } else {
        Err(de::Error::custom(format!("Invalid Issue Year: {}", year)))
    }
}

fn validate_byr<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let year = usize::deserialize(deserializer)?;

    if year >= 1920 && year <= 2002 {
        return Ok(year);
    } else {
        Err(de::Error::custom(format!("Invalid birth year: {}", year)))
    }
}

fn validate_eyr<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    let year = usize::deserialize(deserializer)?;

    if year >= 2020 && year <= 2030 {
        return Ok(year);
    } else {
        Err(de::Error::custom(format!(
            "Invalid expiration  year: {}",
            year
        )))
    }
}

fn validate_pid<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let pid = String::deserialize(deserializer)?;

    if pid.len() == 9 && pid.chars().all(|c| c.is_digit(10)) {
        Ok(pid)
    } else {
        Err(de::Error::custom(format!("Invalid pid: {}", pid)))
    }
}

fn validate_hcl<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let color = String::deserialize(deserializer)?;

    if color.len() == 7
        && color.starts_with('#')
        && color.chars().skip(1).all(|c| c.is_ascii_hexdigit())
    {
        Ok(color)
    } else {
        Err(de::Error::custom(format!("Invalid color: {}", color)))
    }
}

fn validate_ecl<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let color = String::deserialize(deserializer)?;
    let allowed_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    if allowed_colors.contains(&color.as_str()) {
        Ok(color)
    } else {
        Err(de::Error::custom(format!("Invalid color: {}", color)))
    }
}

pub fn preprocess_input(input: &str) -> Vec<String> {
    input
        .split("\n\n") // Split by double newline
        .map(|block| block.replace("\n", " ").replace(":", "=").replace(" ", "&"))
        .collect()
}
