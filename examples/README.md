# Polywrap Rust Client Examples

## Hello world
Invokes the logger wrap, which interacts with the logger plugin. It shows a println message from WASM world
```shell
cargo run --example hello-world --release
```

## File system
Invokes the File System plugin; which creats, reads and deletes a file
```shell
cargo run --example fs --release
```

## Http
Invokes the HTTP plugin, doing GET and POST requests
```shell
cargo run --example http --release
```

<!-- 2. Ethereum

```shell
cargo run --example ethereum --release
``` -->