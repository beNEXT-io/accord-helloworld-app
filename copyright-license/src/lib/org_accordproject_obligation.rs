use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::org_accordproject_runtime::*;
use crate::org_accordproject_money::*;
use crate::concerto_1_0_0::*;
use crate::org_accordproject_contract::*;
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentObligation {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "amount",
   )]
   pub amount: MonetaryAmount,
   
   #[serde(
      rename = "description",
   )]
   pub description: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
   
   #[serde(rename = "contract")]
   pub contract: Contract,
   
   #[serde(rename = "promisor")]
   pub promisor: Option<Participant>,
   
   #[serde(rename = "promisee")]
   pub promisee: Option<Participant>,
   
   #[serde(
      rename = "deadline",
      skip_serializing_if = "Option::is_none",
      serialize_with = "serialize_datetime_option",
      deserialize_with = "deserialize_datetime_option",
   )]
   pub deadline: Option<DateTime<Utc>>,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub _timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationObligation {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "title",
   )]
   pub title: String,
   
   #[serde(
      rename = "message",
   )]
   pub message: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
   
   #[serde(rename = "contract")]
   pub contract: Contract,
   
   #[serde(rename = "promisor")]
   pub promisor: Option<Participant>,
   
   #[serde(rename = "promisee")]
   pub promisee: Option<Participant>,
   
   #[serde(
      rename = "deadline",
      skip_serializing_if = "Option::is_none",
      serialize_with = "serialize_datetime_option",
      deserialize_with = "deserialize_datetime_option",
   )]
   pub deadline: Option<DateTime<Utc>>,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub _timestamp: DateTime<Utc>,
}

