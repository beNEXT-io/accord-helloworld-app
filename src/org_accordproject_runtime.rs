use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Obligation {
    #[serde(rename = "$class")]
    pub class: String,

    #[serde(rename = "contract")]
    pub contract: String,

    #[serde(rename = "deadline")]
    pub deadline: Option<String>,

    #[serde(rename = "promisee")]
    pub promisee: Option<String>,

    #[serde(rename = "promisor")]
    pub promisor: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// The class identifier for org.accordproject.runtime.Request
    #[serde(rename = "$class")]
    pub class: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    /// The class identifier for org.accordproject.runtime.Response
    #[serde(rename = "$class")]
    pub class: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    /// The class identifier for org.accordproject.runtime.State
    #[serde(rename = "$class")]
    pub class: String,
}
