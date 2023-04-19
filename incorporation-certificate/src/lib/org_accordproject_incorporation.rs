use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::lib::org_accordproject_contract::*;
use crate::lib::org_accordproject_signature::*;
use crate::lib::concerto_1_0_0::*;
use crate::lib::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct IncorporationCertificate {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "companyName",
   )]
   pub company_name: String,
   
   #[serde(
      rename = "incorporationState",
   )]
   pub incorporation_state: String,
   
   #[serde(
      rename = "streetAddress",
   )]
   pub street_address: String,
   
   #[serde(
      rename = "addressRegion",
   )]
   pub address_region: String,
   
   #[serde(
      rename = "addressLocality",
   )]
   pub address_locality: String,
   
   #[serde(
      rename = "postalCode",
   )]
   pub postal_code: String,
   
   #[serde(
      rename = "registeredAgentName",
   )]
   pub registered_agent_name: String,
   
   #[serde(
      rename = "incorporationDate",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub incorporation_date: DateTime<Utc>,
   
   #[serde(
      rename = "authorizedShareCapital",
   )]
   pub authorized_share_capital: u64,
   
   #[serde(
      rename = "parValue",
   )]
   pub par_value: f64,
   
   #[serde(
      rename = "incorporatorName",
   )]
   pub incorporator_name: String,
   
   #[serde(
      rename = "incorporatorAddress",
   )]
   pub incorporator_address: String,
   
   #[serde(
      rename = "incorporatorCity",
   )]
   pub incorporator_city: String,
   
   #[serde(
      rename = "incorporatorState",
   )]
   pub incorporator_state: String,
   
   #[serde(
      rename = "incorporatorZip",
   )]
   pub incorporator_zip: String,
   
   #[serde(
      rename = "contractId",
   )]
   pub contract_id: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "companyName",
   )]
   pub company_name: String,
   
   #[serde(
      rename = "incorporationDate",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub incorporation_date: DateTime<Utc>,
   
   #[serde(
      rename = "authorizedShareCapital",
   )]
   pub authorized_share_capital: u64,
   
   #[serde(
      rename = "parValue",
   )]
   pub par_value: f64,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub _timestamp: DateTime<Utc>,
}

