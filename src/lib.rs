use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Welcome {
    #[serde(rename = "$schema")]
    schema: String,
    definitions: Definitions,
}

#[derive(Serialize, Deserialize)]
pub struct Definitions {
    #[serde(rename = "org.accordproject.helloworld.MyRequest")]
    org_accordproject_helloworld_my_request: OrgAccordprojectHelloworldMyRequest,
    #[serde(rename = "org.accordproject.helloworld.MyResponse")]
    org_accordproject_helloworld_my_response: OrgAccordprojectHelloworldMyResponse,
    #[serde(rename = "org.accordproject.helloworld.HelloWorldClause")]
    org_accordproject_helloworld_hello_world_clause: OrgAccordprojectHelloworldHelloWorldClause,
    #[serde(rename = "org.accordproject.runtime.Request")]
    org_accordproject_runtime_request: OrgAccordprojectRuntime,
    #[serde(rename = "org.accordproject.runtime.Response")]
    org_accordproject_runtime_response: OrgAccordprojectRuntime,
    #[serde(rename = "org.accordproject.runtime.Obligation")]
    org_accordproject_runtime_obligation: OrgAccordprojectRuntimeObligation,
    #[serde(rename = "org.accordproject.runtime.State")]
    org_accordproject_runtime_state: OrgAccordprojectRuntime,
    #[serde(rename = "org.accordproject.contract.Contract")]
    org_accordproject_contract_contract: OrgAccordprojectContractContract,
    #[serde(rename = "org.accordproject.contract.Clause")]
    org_accordproject_contract_clause: OrgAccordprojectContractClause,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectContractClause {
    title: String,
    description: String,
    #[serde(rename = "type")]
    org_accordproject_contract_clause_type: String,
    properties: OrgAccordprojectContractClauseProperties,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectContractClauseProperties {
    #[serde(rename = "$class")]
    class: Class,
    #[serde(rename = "clauseId")]
    clause_id: ClauseId,
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    #[serde(rename = "type")]
    class_type: String,
    #[serde(rename = "default")]
    class_default: String,
    pattern: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ClauseId {
    #[serde(rename = "type")]
    clause_id_type: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectContractContract {
    title: String,
    description: String,
    #[serde(rename = "type")]
    org_accordproject_contract_contract_type: String,
    properties: OrgAccordprojectContractContractProperties,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectContractContractProperties {
    #[serde(rename = "$class")]
    class: Class,
    #[serde(rename = "contractId")]
    contract_id: ClauseId,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectHelloworldHelloWorldClause {
    title: String,
    description: String,
    #[serde(rename = "type")]
    org_accordproject_helloworld_hello_world_clause_type: String,
    properties: OrgAccordprojectHelloworldHelloWorldClauseProperties,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectHelloworldHelloWorldClauseProperties {
    #[serde(rename = "$class")]
    class: Class,
    name: Name,
    #[serde(rename = "clauseId")]
    clause_id: ClauseId,
}

#[derive(Serialize, Deserialize)]
pub struct Name {
    #[serde(rename = "type")]
    name_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectHelloworldMyRequest {
    title: String,
    description: String,
    #[serde(rename = "type")]
    org_accordproject_helloworld_my_request_type: String,
    properties: OrgAccordprojectHelloworldMyRequestProperties,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectHelloworldMyRequestProperties {
    #[serde(rename = "$class")]
    class: Class,
    input: Name,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectHelloworldMyResponse {
    title: String,
    description: String,
    #[serde(rename = "type")]
    org_accordproject_helloworld_my_response_type: String,
    properties: OrgAccordprojectHelloworldMyResponseProperties,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectHelloworldMyResponseProperties {
    #[serde(rename = "$class")]
    class: Class,
    output: Name,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectRuntimeObligation {
    title: String,
    description: String,
    #[serde(rename = "type")]
    org_accordproject_runtime_obligation_type: String,
    properties: OrgAccordprojectRuntimeObligationProperties,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectRuntimeObligationProperties {
    #[serde(rename = "$class")]
    class: Class,
    contract: ClauseId,
    promisor: ClauseId,
    promisee: ClauseId,
    deadline: Deadline,
}

#[derive(Serialize, Deserialize)]
pub struct Deadline {
    format: String,
    #[serde(rename = "type")]
    deadline_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectRuntime {
    title: String,
    description: String,
    #[serde(rename = "type")]
    org_accordproject_runtime_type: String,
    properties: OrgAccordprojectRuntimeRequestProperties,
    required: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OrgAccordprojectRuntimeRequestProperties {
    #[serde(rename = "$class")]
    class: Class,
}
