use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, TimeZone, Utc };
   
use crate::lib::org_accordproject_contract::*;
use crate::lib::org_accordproject_party::*;
use crate::lib::org_accordproject_runtime::*;
use crate::lib::org_accordproject_obligation::*;
use crate::lib::org_accordproject_money::*;
use crate::lib::org_accordproject_signature::*;
use crate::lib::org_accordproject_payment::*;
use crate::lib::concerto_1_0_0::*;
use crate::lib::utils::*;
   
#[derive(Debug, Serialize, Deserialize)]
pub struct FullPaymentUponSignatureState {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(
      rename = "status",
   )]
   pub status: ContractLifecycleStatus,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ContractLifecycleStatus {
   INITIALIZED,
   OBLIGATION_EMITTED,
   COMPLETED,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullPaymentUponSignatureContract {
   #[serde(
      rename = "$class",
   )]
   pub _class: String,
   
   #[serde(rename = "buyer")]
   pub buyer: Party,
   
   #[serde(rename = "seller")]
   pub seller: Party,
   
   #[serde(
      rename = "amount",
   )]
   pub amount: MonetaryAmount,
   
   #[serde(
      rename = "contractId",
   )]
   pub contract_id: String,
   
   #[serde(
      rename = "$identifier",
   )]
   pub _identifier: String,
}

