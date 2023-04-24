use std::fs::File;
use std::io::Read;
mod lib;
use lib::org_accordproject_payment_fulluponsignature::*;
use lib::org_accordproject_signature::*;
use serde_json::json;
use serde_json::Value;
use std::process::Command;

use crate::lib::org_accordproject_obligation::NotificationObligation;
use crate::lib::org_accordproject_obligation::PaymentObligation;

fn handle_contract() -> String {
    r#"
        {
            "$class": "org.accordproject.payment.fulluponsignature.FullPaymentUponSignatureContract",
            "buyer": "resource:org.accordproject.party.Party#Dan",
            "seller": "resource:org.accordproject.party.Party#Steve",
            "amount": {
                "$class": "org.accordproject.money.MonetaryAmount",
                "doubleValue": 0.01,
                "currencyCode": "USD"
            },
            "contractId": "8e844b8d-f26e-4796-b5e1-c33016d6a76b",
            "$identifier": "8e844b8d-f26e-4796-b5e1-c33016d6a76b"
        }
    "#
    .to_owned()
}

fn handle_party() -> String {
    let cmd = Command::new("concerto")
        .args([
            "generate",
            "sample",
            "--model",
            "https://models.accordproject.org/accordproject/party.cto",
            "--concept",
            "org.accordproject.party.Party",
        ])
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&cmd.stdout).into_owned()
}

fn get_struct(value: String) -> String {
    value.split("#").collect::<Vec<&str>>()[0]
        .split(".")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap_or_default()
        .to_owned()
}

fn get_relationship_as_json(name: &serde_json::Value) -> std::io::Result<serde_json::Value> {
    println!("{:?}", name);
    let mut sender: serde_json::Value;
    if let Value::String(value) = name {
        let string_json = match get_struct(value.clone()).as_str() {
            "FullPaymentUponSignatureContract" => handle_contract(),
            "Party" => handle_party(),
            other => panic!(
                "{}",
                format!(
                    "The {:?} value is not handled by the relationship to json handler",
                    other
                )
            ),
        };
        sender = serde_json::from_str(&string_json)?;
    } else {
        panic!("The required field is not handled by the relationship handler")
    };

    if let Some(identifier) = sender.get_mut("$identifier") {
        for (i, part) in name.to_string().replace('"', "").split("#").enumerate() {
            if i == 1 {
                *identifier = part.replace("%20", " ").into()
            }
        }
    }

    return Ok(sender);
}

fn main() -> std::io::Result<()> {
    // Pull the json request object from file.
    let mut file = File::open("./full-upon-signature/model/request.json")?;
    let mut request_json = String::new();
    file.read_to_string(&mut request_json)?;

    // Let's see what we have.
    println!("request_json = {}", request_json);

    // Deserialise MyRequest
    let mut request: serde_json::Value =
        serde_json::from_str(&request_json).unwrap_or_else(|err| {
            // Ooops!  Didn't work - display an error and exit.
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });

    if let Some(name) = request.get_mut("contract") {
        if let Ok(json) = get_relationship_as_json(&name) {
            *name = json;
        }
    }
    if let Some(name) = request.get_mut("promiser") {
        if let Ok(json) = get_relationship_as_json(&name) {
            *name = json;
        }
    }

    if let Some(name) = request.get_mut("promisee") {
        if let Ok(json) = get_relationship_as_json(&name) {
            *name = json;
        }
    }

    let response: NotificationObligation = serde_json::from_str(&request.to_string())
        .unwrap_or_else(|err| {
            // Ooops!  Didn't work - display an error and exit.
            eprintln!("Error: {}", err);
            std::process::exit(1);
        });

    println!("response_json = {:?}", response);

    Ok(())
}
