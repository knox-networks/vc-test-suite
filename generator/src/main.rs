use std::str::FromStr;

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum GenerationTarget {
    #[serde(rename = "credential")]
    Credential,
    #[serde(rename = "presentation")]
    Presentation,
}

impl FromStr for GenerationTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "credential" => Ok(GenerationTarget::Credential),
            "presentation" => Ok(GenerationTarget::Presentation),
            _ => Err(format!("{} is not a valid target", s)),
        }
    }
}

/// Simple program to greet a person
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    file_name: String,

    #[arg(short, long)]
    target: GenerationTarget,
}

fn main() {
    let args = Args::parse();
    let _ = args.target;
    let json = get_credential();
    println!("{}", json.to_string());
}

fn get_credential() -> serde_json::Value {
    json!({
      "@context": [
        "https://www.w3.org/2018/credentials/v1",
        "https://www.w3.org/2018/credentials/examples/v1"
      ],
      "id": "https://bank.com/credentials/83627465",
      "type": [
        "VerifiableCredential",
        "BankCard"
      ],
      "issuer": "did:knox:z9j11k9soh9kJ1vD9pYR87ZhD7zE1U7ZA3XVSkWjY4YLg",
      "issuanceDate": "2022-04-11T16:36:24Z",
      "subject": {
        "account": "000-000-204",
        "address": "19 Knox St, Toronto, ON",
        "birthDate": "1981-04-01",
        "branch": "C09",
        "country": "Canada",
        "familyName": "Kim",
        "gender": "Male",
        "givenName": "Francis",
        "id": "did:knox:z6MkgLDqhuPFrED5w8PuMjKk1pMMamH11PnmES51Pp348c81",
        "phone": "416-984-1234",
        "type": [
          "BankCard"
        ]
      },
      "proof": {
        "type": "Ed25519Signature2020",
        "created": "2022-04-11T16:36:24Z",
        "verificationMethod": "did:knox:z9j11k9soh9kJ1vD9pYR87ZhD7zE1U7ZA3XVSkWjY4YLg#z9j11k9soh9kJ1vD9pYR87ZhD7zE1U7ZA3XVSkWjY4YLg",
        "proofPurpose": "assertionMethod",
        "proofValue": "z4xTXcWHhZY8oXCXTKSw3N9qmRKjQAUUVbNnQz1FqKCAYiGieYohBRcSKGK9YcBuKqyqzjbaohmtMZBAenC9huBJ"
      }
    }
    )
}
