mod logic;

use crate::logic::logic::*;
use lambda_http::{run, service_fn, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {

    use super::*;

    use helloworld::concerto_at_1_0_0::*;
    use helloworld::org_accordproject_contract::*;
    use helloworld::org_accordproject_runtime::*;

    #[test]
    fn check_my_request() {
        let input_json = r#"{    
            "$class": "org.accordproject.helloworld.MyRequest",
            "input": "Tech Working Group"
        }"#;
        let _: MyRequest = serde_json::from_str(&input_json).unwrap();
    }

    #[test]
    fn check_my_response() {
        let input_json = r#"{    
            "$class": "org.accordproject.helloworld.MyResponse",
            "output": "Hello Fred Blogs"
        }"#;
        let _: MyResponse = serde_json::from_str(&input_json).unwrap();
    }

    #[test]
    fn check_hello_world_clause() {
        let input_json = r#"{    
            "$class": "org.accordproject.helloworld.HelloWorldClause",
            "clauseId": "Some clause id",
            "name": "Some Name"
        }"#;
        let _: HelloWorldClause = serde_json::from_str(&input_json).unwrap();
    }

    #[test]
    fn check_hello_world() {
        let input_json = r#"{
            "clause" : {
                "$class": "org.accordproject.helloworld.Contract",
                "clauseId": "Some clause id"
            },
            "contract" : {
                "$class": "org.accordproject.helloworld.Contract",
                "contractId": "Some contract id"
            },
            "helloWorldClause" : {    
                "$class": "org.accordproject.helloworld.HelloWorldClause",
                "clauseId": "Some clause id",
                "name": "Some Name"
            },
            "MyRequest" : {    
                "$class": "org.accordproject.helloworld.MyRequest",
                "input": "Tech Working Group"
            },
            "MyResponse" : {    
                "$class": "org.accordproject.helloworld.MyResponse",
                "output": "Hello Fred Blogs"
            },
            "obligation" : {    
                "$class": "org.accordproject.helloworld.Obligation",
                "contract": "Some contract",
                "deadline": "Some deadline",
                "promisee": "Some promisee",
                "promisor": "Some promisor"
            },
            "request" : {
                "$class": "org.accordproject.helloworld.Request"
            },
            "response" : {
                "$class": "org.accordproject.helloworld.Response"
            },
            "state" : {    
                "$class": "org.accordproject.helloworld.State"
            }
        }"#;
        let _: HelloWorld = serde_json::from_str(input_json).unwrap();
    }

    #[test]
    fn check_request() {
        let input_json = r#"{    
            "$class": "org.accordproject.helloworld.Request"
        }"#;
        let _: Request = serde_json::from_str(&input_json).unwrap();
    }

    #[test]
    fn check_response() {
        let input_json = r#"{    
            "$class": "org.accordproject.helloworld.Response"
        }"#;
        let _: Response = serde_json::from_str(&input_json).unwrap();
    }

    #[test]
    fn check_state() {
        let input_json = r#"{    
            "$class": "org.accordproject.helloworld.State"
        }"#;
        let _: State = serde_json::from_str(&input_json).unwrap();
    }

    #[test]
    fn check_obligation() {
        let input_json = r#"{    
            "$class": "org.accordproject.helloworld.Obligation",
            "contract": "Some contract",
            "deadline": "Some deadline",
            "promisee": "Some promisee",
            "promisor": "Some promisor"
        }"#;
        let _: Obligation = serde_json::from_str(&input_json).unwrap();
    }

    #[test]
    fn check_clause() {
        let input_json = r#"{
            "$class": "org.accordproject.helloworld.Contract",
            "clauseId": "Some clause id"
        }"#;
        let _: Clause = serde_json::from_str(input_json).unwrap();
    }

    #[test]
    fn check_contract() {
        let input_json = r#"{
            "$class": "org.accordproject.helloworld.Contract",
            "contractId": "Some contract id"
        }"#;
        let _: Contract = serde_json::from_str(input_json).unwrap();
    }

    #[test]
    fn check_asset() {
        let input_json = r#"{
            "Concept": {},
            "Identifier": "Some identifier"
        }"#;
        let _: Asset = serde_json::from_str(input_json).unwrap();
    }

    #[test]
    fn check_participant() {
        let input_json = r#"{
            "Concept": {},
            "Identifier": "Some identifier"
        }"#;
        let _: Participant = serde_json::from_str(input_json).unwrap();
    }

    #[test]
    fn check_transaction() {
        let input_json = r#"{
            "Concept": {},
            "Timestamp": "2023-02-18T18:30:15.123Z"
        }"#;
        let _: Transaction = serde_json::from_str(input_json).unwrap();
    }

    #[test]
    fn check_event() {
        let input_json = r#"{
            "Concept": {},
            "Timestamp": "2023-02-18T18:30:15.123Z"
        }"#;
        let _: Event = serde_json::from_str(input_json).unwrap();
    }
}
