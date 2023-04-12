use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct Concept {
   #[serde(
      rename = "$class",
   )]
   pub class: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
   #[serde(
      rename = "$class",
   )]
   pub class: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
   #[serde(
      rename = "$class",
   )]
   pub class: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
   #[serde(
      rename = "$class",
   )]
   pub class: String,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
   #[serde(
      rename = "$class",
   )]
   pub class: String,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub timestamp: DateTime<Utc>,
}

