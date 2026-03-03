use crate::units::error::UnitError;

pub fn split_value_and_unit(input: &str) -> Result<(f64, String), UnitError> {
    let trimmed = input.trim();
    let split_idx = trimmed
        .find(|c: char| !c.is_numeric() && c != '.' && c != '-' && c != '+' && c != 'e' && c != 'E')
        .unwrap_or(trimmed.len());
    let (num_part, unit_part) = trimmed.split_at(split_idx);
    let value: f64 = num_part.trim().parse().map_err(|_| {
        UnitError::ParseError(format!("Could not parse numeric value from '{input}'"))
    })?;
    Ok((value, unit_part.trim().to_string()))
}
