use serde::{Deserializer, Deserialize};
pub fn nullable_string<'de, D>(deserializer: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or("".to_string()))
}
