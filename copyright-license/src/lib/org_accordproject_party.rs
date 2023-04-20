use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::lib::concerto_1_0_0::*;
use crate::lib::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct Party {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "partyId",
   )]
   pub party_id: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
}

