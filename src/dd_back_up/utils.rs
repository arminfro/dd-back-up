use chrono::Local;

pub fn current_date() -> String {
    let current_date = Local::now();
    current_date.format("%Y-%m-%d").to_string()
}

/// Converts a size string with unit suffix (e.g., "100M", "16G") to the equivalent size in bytes.
/// Returns the converted size as a `Result<u64, String>`. If the conversion fails, an error message
/// is returned as `String`.
pub fn convert_to_byte_size(size_str: &str) -> Result<Option<u64>, String> {
    let size_str = size_str.trim();
    let unit = size_str.chars().last().ok_or("")?;
    let size_of_unit = size_str[..size_str.len() - 1]
        .parse::<f64>()
        .map_err(|e| format!("Error parsing unit size: {}", e))?;

    let size: Option<f64> = match unit {
        'K' => Some(size_of_unit * 1024.0),
        'M' => Some(size_of_unit * 1024.0 * 1024.0),
        'G' => Some(size_of_unit * 1024.0 * 1024.0 * 1024.0),
        'T' => Some(size_of_unit * 1024.0 * 1024.0 * 1024.0 * 1024.0),
        _ => None,
    };

    if let Some(size) = size {
        Ok(Some(size.round() as u64))
    } else {
        Ok(None)
    }
}
