use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::org_accordproject_runtime::*;
use crate::org_accordproject_contract::*;
use crate::concerto_1_0_0::*;
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct ContractSigned {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(rename = "contract")]
   pub contract: Contract,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub _timestamp: DateTime<Utc>,
}

