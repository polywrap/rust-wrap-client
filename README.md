![polywrap-banner](https://raw.githubusercontent.com/polywrap/branding/master/assets/banner.png)

# Rust Client [![codecov](https://codecov.io/gh/polywrap/rust-client/branch/main/graph/badge.svg?token=Z0VNH4R5UR)](https://codecov.io/gh/polywrap/rust-client)

Implementation of the Polywrap client in Rust.

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
polywrap = 0.1.6
```

## Getting started

Create a new Polywrap Client Config Builder instance, add the bundles you want to use, and then create a new Polywrap Client instance from the builder.

```rust
use polywrap::*;

#[derive(Serialize)]
struct CatArgs {
    cid: String,
    #[serde(rename = "ipfsProvider")]
    ipfs_provider: String,
}

fn main() {
    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());
    let client = PolywrapClient::new(config.build());

    let result = client.invoke::<ByteBuf>(
        uri!("wrapscan.io/polywrap/ipfs-client@1.0"),
        "cat",
        Some(&to_vec(
            &CatArgs {
                cid: "QmbWqxBEKC3P8tqsKc98xmWNzrzDtRLMiMPL8wBuTGsMnR".to_string(),
                ipfs_provider: "https://ipfs.io".to_string(),
            }
        ).unwrap()),
        None,
        None
    );

    if result.is_err() {
        // Handle error
    };

    println!(
        "Cat Result: {}",
        String::from_utf8(result.unwrap().to_vec()).unwrap()
    );
}
```

## Resources

- [Documentation](https://docs.polywrap.io/)
- [Examples](./examples/)
- [Features supported](https://github.com/polywrap/client-readiness/tree/main/clients/rs/src/features)
- [Support](https://discord.polywrap.io)

## Contributions

Please check out our [contributing guide](./CONTRIBUTING.md) for guidelines about how to proceed.