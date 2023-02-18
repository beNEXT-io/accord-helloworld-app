use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Clause {
    #[serde(rename = "$class")]
    pub class: String,
    #[serde(rename = "clauseId")]
    pub clause_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    #[serde(rename = "$class")]
    pub class: String,
    #[serde(rename = "contractId")]
    pub contract_id: String,
}
