use std::fs::File;
use std::io::Read;
use lib::org_accordproject_acceptanceofdelivery_0_15_0::*;
use std::process::Command;

fn get_relationship_as_json(name:&serde_json::Value) -> std::io::Result<serde_json::Value>  {
    let cmd = Command::new("concerto")
    .args(["generate", "sample","--model","https://models.accordproject.org/organization.cto","--concept","org.accordproject.organization.Organization"])
    .output()
    .expect("failed to execute process");


    let string_json = String::from_utf8_lossy(&cmd.stdout);

    let mut sender:serde_json::Value = serde_json::from_str(&string_json)?;

    if let Some(identifier) = sender.get_mut("$identifier") {

    for (i, part) in name.to_string().replace('"',"").split("#").enumerate() {
            if i == 1 {
                *identifier = part.replace("%20", " ").into()
            }
        }
    }

    return Ok(sender);

}

fn main() -> std::io::Result<()> {
    // Pull the json request object from file.
    let mut file = File::open("./acceptance-of-delivery-clause/model/request.json")?;
    let mut request_json = String::new();
    file.read_to_string(&mut request_json)?;

    // Let's see what we have.
    println!("request_json = {}", request_json);

    // Deserialise MyRequest
    let mut request: serde_json::Value = serde_json::from_str(&request_json).unwrap_or_else(|err| {
        // Ooops!  Didn't work - display an error and exit.
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    if let Some(name) = request.get_mut("shipper") {
        if let Ok(json) = get_relationship_as_json(&name) {
           *name = json;

        }
    }

    if let Some(name) = request.get_mut("receiver") {
        if let Ok(json) = get_relationship_as_json(&name) {
            *name = json;
         }
    }

    let response: AcceptanceOfDeliveryClause = serde_json::from_str(&request.to_string()).unwrap_or_else(|err| {
        // Ooops!  Didn't work - display an error and exit.
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    println!("response_json = {:?}", response);

    Ok(())
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
