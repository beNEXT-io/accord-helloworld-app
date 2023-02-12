use std::fs::File;
use std::io::Read;

use helloworld::OrgAccordprojectHelloworldMyRequestProperties;
use serde_json::Error;

fn main() -> std::io::Result<()> {
    // Pull the json request object from file.
    let mut file = File::open("./model/request.json")?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    // Let's see what we have.
    println!("{}", json);

    // Attempt to parse the request object into the corresponding Struct.
    let request: Result<OrgAccordprojectHelloworldMyRequestProperties, Error> =
        serde_json::from_str(&json);

    // Let's see if that worked or not.
    let _ = request.unwrap_or_else(|err| {
        // Ooops!  Didn't work - display an error and exit.
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    // Yay! Worked.
    println!("OK: The parse request returned an the Ok condition");

    Ok(())
}
