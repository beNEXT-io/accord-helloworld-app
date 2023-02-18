use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "Concept")]
    pub concept: Concept,
}
