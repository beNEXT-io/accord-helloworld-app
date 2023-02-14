use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Helloworld {
    #[serde(rename = "clause")]
    pub clause: Option<Clause>,

    #[serde(rename = "contract")]
    pub contract: Option<Contract>,

    #[serde(rename = "helloWorldClause")]
    pub hello_world_clause: Option<HelloWorldClause>,

    #[serde(rename = "MyRequest")]
    pub my_request: Option<MyRequest>,

    #[serde(rename = "myResponse")]
    pub my_response: Option<MyResponse>,

    #[serde(rename = "obligation")]
    pub obligation: Option<Obligation>,

    #[serde(rename = "request")]
    pub request: Option<Request>,

    #[serde(rename = "response")]
    pub response: Option<Response>,

    #[serde(rename = "state")]
    pub state: Option<State>,
}

/// An instance of org.accordproject.contract.Clause
#[derive(Debug, Serialize, Deserialize)]
pub struct Clause {
    /// The class identifier for org.accordproject.contract.Clause
    #[serde(rename = "$class")]
    pub class: String,

    /// The instance identifier for this type
    #[serde(rename = "clauseId")]
    pub clause_id: String,
}

/// An instance of org.accordproject.contract.Contract
#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    /// The class identifier for org.accordproject.contract.Contract
    #[serde(rename = "$class")]
    pub class: String,

    /// The instance identifier for this type
    #[serde(rename = "contractId")]
    pub contract_id: String,
}

/// An instance of org.accordproject.helloworld.HelloWorldClause
#[derive(Debug, Serialize, Deserialize)]
pub struct HelloWorldClause {
    /// The class identifier for org.accordproject.helloworld.HelloWorldClause
    #[serde(rename = "$class")]
    pub class: String,

    /// The instance identifier for this type
    #[serde(rename = "clauseId")]
    pub clause_id: String,

    #[serde(rename = "name")]
    pub name: String,
}

/// An instance of org.accordproject.helloworld.MyRequest
#[derive(Debug, Serialize, Deserialize)]
pub struct MyRequest {
    /// The class identifier for org.accordproject.helloworld.MyRequest
    #[serde(rename = "$class")]
    pub class: String,

    #[serde(rename = "input")]
    pub input: String,
}

/// An instance of org.accordproject.helloworld.MyResponse
#[derive(Debug, Serialize, Deserialize)]
pub struct MyResponse {
    /// The class identifier for org.accordproject.helloworld.MyResponse
    #[serde(rename = "$class")]
    pub class: String,

    #[serde(rename = "output")]
    pub output: String,
}

/// An instance of org.accordproject.runtime.Obligation
#[derive(Debug, Serialize, Deserialize)]
pub struct Obligation {
    /// The class identifier for org.accordproject.runtime.Obligation
    #[serde(rename = "$class")]
    pub class: String,

    /// The identifier of an instance of org.accordproject.contract.Contract
    #[serde(rename = "contract")]
    pub contract: String,

    #[serde(rename = "deadline")]
    pub deadline: Option<String>,

    /// The identifier of an instance of concerto@1.0.0.Participant
    #[serde(rename = "promisee")]
    pub promisee: Option<String>,

    /// The identifier of an instance of concerto@1.0.0.Participant
    #[serde(rename = "promisor")]
    pub promisor: Option<String>,
}

/// An instance of org.accordproject.runtime.Request
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// The class identifier for org.accordproject.runtime.Request
    #[serde(rename = "$class")]
    pub class: String,
}

/// An instance of org.accordproject.runtime.Response
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    /// The class identifier for org.accordproject.runtime.Response
    #[serde(rename = "$class")]
    pub class: String,
}

/// An instance of org.accordproject.runtime.State
#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    /// The class identifier for org.accordproject.runtime.State
    #[serde(rename = "$class")]
    pub class: String,
}
