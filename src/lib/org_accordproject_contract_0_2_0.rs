use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::lib::concerto_1_0_0::*;
use crate::lib::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
   #[serde(
      rename = "$class",
   )]
   pub class: String,
   
   #[serde(
      rename = "contractId",
   )]
   pub contract_id: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Clause {
   #[serde(
      rename = "$class",
   )]
   pub class: String,
   
   #[serde(
      rename = "clauseId",
   )]
   pub clause_id: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub identifier: String,
}

