use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::org_accordproject_contract::*;
use crate::org_accordproject_party::*;
use crate::org_accordproject_runtime::*;
use crate::org_accordproject_obligation::*;
use crate::org_accordproject_money::*;
use crate::org_accordproject_organization::*;
use crate::concerto_1_0_0::*;
use crate::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub _timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PayOut {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "amount",
   )]
   pub amount: MonetaryAmount,
   
   #[serde(
      rename = "$timestamp",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub _timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyrightLicenseContract {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "effectiveDate",
      serialize_with = "serialize_datetime",
      deserialize_with = "deserialize_datetime",
   )]
   pub effective_date: DateTime<Utc>,
   
   #[serde(rename = "licensee")]
   pub licensee: Party,
   
   #[serde(
      rename = "licenseeState",
   )]
   pub licensee_state: String,
   
   #[serde(
      rename = "licenseeEntityType",
   )]
   pub licensee_entity_type: String,
   
   #[serde(
      rename = "licenseeAddress",
   )]
   pub licensee_address: String,
   
   #[serde(rename = "licensor")]
   pub licensor: Party,
   
   #[serde(
      rename = "licensorState",
   )]
   pub licensor_state: String,
   
   #[serde(
      rename = "licensorEntityType",
   )]
   pub licensor_entity_type: String,
   
   #[serde(
      rename = "licensorAddress",
   )]
   pub licensor_address: String,
   
   #[serde(
      rename = "territory",
   )]
   pub territory: String,
   
   #[serde(
      rename = "purposeDescription",
   )]
   pub purpose_description: String,
   
   #[serde(
      rename = "workDescription",
   )]
   pub work_description: String,
   
   #[serde(
      rename = "paymentClause",
   )]
   pub payment_clause: PaymentClause,
   
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
pub struct PaymentClause {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "amountText",
   )]
   pub amount_text: String,
   
   #[serde(
      rename = "amount",
   )]
   pub amount: MonetaryAmount,
   
   #[serde(
      rename = "paymentProcedure",
   )]
   pub payment_procedure: String,
   
   #[serde(
      rename = "clauseId",
   )]
   pub clause_id: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
}

