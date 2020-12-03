use serde::{Serialize, Deserialize, Deserializer};
use crate::domain::models::serialization_helpers::nullable_string;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Site {
    #[serde(rename="SiteId", deserialize_with="nullable_string")]
    pub site_id: String,
    #[serde(rename="IsTastingStore")]
    pub is_tasting_store: bool,
    #[serde(rename="Alias", deserialize_with="nullable_string")]
    pub alias: String,
    #[serde(rename="Address", deserialize_with="nullable_string")]
    pub address: String,
    #[serde(rename="DisplayName", deserialize_with="nullable_string")]
    pub display_name: String,
    #[serde(rename="PostalCode", deserialize_with="nullable_string")]
    pub postal_code: String,
    #[serde(rename="City", deserialize_with="nullable_string")]
    pub city: String,
    #[serde(rename="County", deserialize_with="nullable_string")]
    pub county: String,
    #[serde(rename="Country", deserialize_with="nullable_string")]
    pub country: String,
    #[serde(rename="IsStore")]
    pub is_store: bool,
    #[serde(rename="IsAgent")]
    pub is_agent: bool,
    #[serde(rename="IsActiveForAgentOrder")]
    pub is_active_for_agent_order: bool,
    #[serde(rename="Phone", deserialize_with="nullable_string")]
    pub phone: String,
    #[serde(rename="Email", deserialize_with="nullable_string")]
    pub email: String,
    #[serde(rename="Services", deserialize_with="nullable_string")]
    pub services: String,
    #[serde(rename="OpeningHours", deserialize_with="empty_opening")]
    pub opening_hours: Vec<OpeningTime>,
    #[serde(rename="Depot", deserialize_with="nullable_string")]
    pub depot: String,
    #[serde(rename="Name", deserialize_with="nullable_string")]
    pub name: String,
    #[serde(rename="Position", deserialize_with="empty_position")]
    pub position: Position,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct OpeningTime {
    #[serde(rename="IsOpen")]
    pub is_open: bool,
    #[serde(rename="Reason", deserialize_with="nullable_string")]
    pub reason: String,
    #[serde(rename="Date", deserialize_with="nullable_string")]
    pub date: String,
    #[serde(rename="OpenFrom", deserialize_with="nullable_string")]
    pub open_from: String,
    #[serde(rename="OpenTo", deserialize_with="nullable_string")]
    pub open_to: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Position {
    #[serde(rename="Lat")]
    pub lat: f64,
    #[serde(rename="Long")]
    pub long: f64,
}


fn empty_opening<'de, D>(deserializer: D) -> Result<Vec<OpeningTime>, D::Error> where D: Deserializer<'de> {
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Vec::new()))
}

fn empty_position<'de, D>(deserializer: D) -> Result<Position, D::Error> where D: Deserializer<'de> {
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Position {lat: 0.0, long: 0.0}))
}