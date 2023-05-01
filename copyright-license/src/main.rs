use std::fs::File;
use std::io::Read;
use lib::org_accordproject_copyrightlicense::*;
use std::process::Command;
use lambda_http::{run, service_fn, Body, Error, Request, Response};


fn get_relationship_as_json(name:&serde_json::Value) -> std::io::Result<serde_json::Value>  {
    let cmd = Command::new("concerto")
    .args(["generate", "sample","--model","https://models.accordproject.org/accordproject/party.cto","--concept","org.accordproject.party.Party"])
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

async fn function_handler(_: Request) -> Result<Response<Body>, Error> {
       // Pull the json request object from file.
    let mut file = File::open("./copyright-license/model/request.json")?;
    let mut request_json = String::new();
    file.read_to_string(&mut request_json)?;

    let mut request: serde_json::Value = serde_json::from_str(&request_json).unwrap_or_else(|err| {
        // Ooops!  Didn't work - display an error and exit.
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    if let Some(name) = request.get_mut("licensee") {
        if let Ok(json) = get_relationship_as_json(&name) {
           *name = json;

        }
    }

    if let Some(name) = request.get_mut("licensor") {
        if let Ok(json) = get_relationship_as_json(&name) {
            *name = json;
         }
    }

    let response: CopyrightLicenseContract = serde_json::from_str(&request.to_string()).unwrap_or_else(|err| {
        // Ooops!  Didn't work - display an error and exit.
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
    
    let resp_as_string = serde_json::to_string(&response).unwrap();


    let resp = Response::builder()
    .status(200)
    .header("content-type",  "application/json")
    .body(resp_as_string.into())
    .map_err(Box::new)?;

    Ok(resp)
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .with_target(false)
    .without_time()
    .init();
    
    run(service_fn(function_handler)).await
}