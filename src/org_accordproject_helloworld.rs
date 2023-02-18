use serde::{Deserialize, Serialize};

use crate::org_accordproject_contract::*;
use crate::org_accordproject_runtime::*;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloWorld {
    #[serde(rename = "clause")]
    pub clause: Option<Clause>,

    #[serde(rename = "contract")]
    pub contract: Option<Contract>,

    #[serde(rename = "helloWorldClause")]
    pub hello_world_clause: Option<HelloWorldClause>,

    #[serde(rename = "MyRequest")]
    pub my_request: Option<MyRequest>,

    #[serde(rename = "MyResponse")]
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
