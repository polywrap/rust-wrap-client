extern crate polywrap;
extern crate serde;

use polywrap::*;
use serde::Serialize;

#[derive(Serialize)]
struct GetContentHashArgs {
    #[serde(rename = "resolverAddress")]
    resolver_address: String,
    domain: String,
}

#[derive(Serialize)]
struct GetResolverArgs {
    #[serde(rename = "registryAddress")]
    registry_address: String,
    domain: String,
}

fn main() {
    let domain = "vitalik.eth".to_string();
    let ens_uri = uri!("wrapscan.io/polywrap/ens@1.0.0");

    let mut config = PolywrapClientConfig::new();
    config
        .add(SystemClientConfig::default().into())
        .add(Web3ClientConfig::default().into());

    let client = Client::new(config.build());

    let resolver_address = client.invoke::<String>(
        &ens_uri,
        "getResolver",
        Some(
            &to_vec(&GetResolverArgs {
                registry_address: "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e".to_string(),
                domain: domain.clone(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if resolver_address.is_err() {
        panic!(
            "Error with get resolver: {}",
            &resolver_address.unwrap_err().to_string()
        )
    }

    println!("Resolver address: {}", resolver_address.clone().unwrap());

    let content_hash = client.invoke::<String>(
        &ens_uri,
        "getContentHash",
        Some(
            &to_vec(&GetContentHashArgs {
                resolver_address: resolver_address.unwrap(),
                domain: domain.clone(),
            })
            .unwrap(),
        ),
        None,
        None,
    );

    if content_hash.is_err() {
        panic!(
            "Error with get content hash: {}",
            &content_hash.unwrap_err().to_string()
        )
    }
    println!(
        "Content hash of {}: {}",
        domain,
        content_hash.clone().unwrap()
    );
}
