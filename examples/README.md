# Polywrap Rust Client Examples

## Hello world
Invokes the logger wrap, which interacts with the logger plugin. It shows a println message from WASM world
```shell
$ cargo run --example hello-world --release
```

## File system
Invokes the File System plugin; which creats, reads and deletes a file
```shell
$ cargo run --example fs --release
```

## Http
Invokes the HTTP plugin, doing GET and POST requests
```shell
$ cargo run --example http --release
```

## Ipfs
Invoke the IPFS Client wrap; adds file to a local IPFS node, and then retrieves it.
Before running this example, you must instantiate a local IPFS node by running the following command:
```
$ npx polywrap infra up --modules=eth-ens-ipfs
```
And now you can run the example:
```shell
$ cargo run --example ipfs --release
```

## Ethers
Invoke the Ethers core & util wraps, and uses the Ethereum Wallet plugin. It gets the balance of the Staking contract and then parses it from Wei to Eth. Also, it executes the sign typed data method
```shell
$ cargo run --example ethers --release
```

## ENS
Invoke the ENS wrap, it gets the resolver & content hash of vitalik.eth
```shell
$ cargo run --example ens --release
```