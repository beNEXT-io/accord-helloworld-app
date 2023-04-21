use std::fs::File;
use std::io::Read;
use lib::org_accordproject_signature::*;

fn get_relationship_as_json(name:&serde_json::Value) -> std::io::Result<serde_json::Value>  {

    let string_json = r#"
    {
        "$class": "org.accordproject.incorporation.IncorporationCertificate",
        "contractId": "",
        "$identifier": ""
    }"#;


    let mut sender:serde_json::Value = serde_json::from_str(&string_json)?;

    if let Some(identifier) = sender.get_mut("$identifier") {

    for (i, part) in name.to_string().replace('"',"").split("#").enumerate() {
            if i == 1 {
                *identifier = part.replace("%20", " ").into()
            }
        }
    }


    if let Some(contract_id) = sender.get_mut("contractId") {

        for (i, part) in name.to_string().replace('"',"").split("#").enumerate() {
                if i == 1 {
                    *contract_id = part.replace("%20", " ").into()
                }
            }
        }

    return Ok(sender);

}


fn main() -> std::io::Result<()> {
    // Pull the json request object from file.
    let mut file = File::open("./incorporation-certificate/model/request.json")?;
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

    if let Some(name) = request.get_mut("contract") {
        if let Ok(json) = get_relationship_as_json(&name) {
           *name = json;
        }
    }

    let response: ContractSigned = serde_json::from_str(&request.to_string()).unwrap_or_else(|err| {
        // Ooops!  Didn't work - display an error and exit.
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });


    println!("response_json = {:?}", response);

    Ok(())
}