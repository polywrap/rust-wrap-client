![polywrap-banner](https://raw.githubusercontent.com/polywrap/branding/master/assets/banner.png)

# Rust Client [![codecov](https://codecov.io/gh/polywrap/rust-client/branch/main/graph/badge.svg?token=Z0VNH4R5UR)](https://codecov.io/gh/polywrap/rust-client)

Implementation of the Polywrap client in Rust.

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
polywrap = 0.1.9
```

## Getting started

Create a new Polywrap Client Config Builder instance, add the bundles you want to use, and then create a new Polywrap Client instance from the builder.

```rust
use polywrap::*;

#[derive(Serialize)]
struct Sha3_256Args {
    message: String,
}

fn main() {
    let mut config = ClientConfig::new();
    config.add(SystemClientConfig::default().into());
    let client = Client::new(config.build());

    let result = client.invoke::<String>(
        &uri!("wrapscan.io/polywrap/sha3@1.0"),
        "sha3_256",
        Some(&to_vec(
            &Sha3_256Args {
                message: "Hello Polywrap!".to_string(),
            }
        ).unwrap()),
        None,
        None
    );

    match result {
        Ok(v) => println!("{}", v),
        Err(e) => panic!("{}", e),
    }
}
```

## Resources

- [Documentation](https://docs.polywrap.io/)
- [Examples](./examples/)
- [Features supported](https://github.com/polywrap/client-readiness/tree/main/clients/rs/src/features)
- [Support](https://discord.polywrap.io)

## Contributions

Please check out our [contributing guide](./CONTRIBUTING.md) for guidelines about how to proceed.