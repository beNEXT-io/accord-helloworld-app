use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Concept {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "Concept")]
    pub concept: Concept,

    #[serde(rename = "Identifier")]
    pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "Concept")]
    pub concept: Concept,

    #[serde(rename = "Identifier")]
    pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "Concept")]
    pub concept: Concept,

    #[serde(
        rename = "Timestamp",
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime"
    )]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "Concept")]
    pub concept: Concept,

    #[serde(
        rename = "Timestamp",
        serialize_with = "serialize_datetime",
        deserialize_with = "deserialize_datetime"
    )]
    pub timestamp: DateTime<Utc>,
}

fn serialize_datetime<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let datetime_str = datetime.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
    serializer.serialize_str(&datetime_str)
}

fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let datetime_str = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S%.3fZ")
        .map_err(serde::de::Error::custom)
}
