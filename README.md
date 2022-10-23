![Public Release Announcement](https://user-images.githubusercontent.com/5522128/177473887-2689cf25-7937-4620-8ca5-17620729a65d.png)

# Polywrap Rust client

## [Polywrap](https://polywrap.io) is a developer tool that enables easy integration of Web3 protocols into any application. It makes it possible for applications on any platform, written in any language, to read and write data to Web3 protocols.

# Working Features

This Polywrap clients enable the execution of WebAssembly Polywrappers (or just “wrappers”) on various environments, regardless of what language this wrapper was built in.

The various clients are built following the functionality of the JavaScript Polywrap Client, which is currently more robust and battle tested, as it has additional capabilities than other MVPs. In the future, the Polywrap DAO will continue improving the various client’s capabilities to reach feature parity with the JS stack, improving the experience in parallel clients for other languages like Python, Go, and Rust.

Here you can see which features have been implemented on each language, and make the decision of which one to use for your project.

| Feature | [Python](https://github.com/polywrap/python-client) | [Javascript](https://github.com/polywrap/toolchain) |  [Go]() | [Rust](https://github.com/polywrap/rust-client) |
| -- | -- | -- | -- | -- |
| **Invoke**  | ✅ | ✅ | | ⚙️|
| Subinvoke | ⚙️ | ✅ | | |
| Interfaces | ❌ | ✅ | | | 
| Env Configuration | ⚙️ | ✅ | | |
| Client Config | ⚙️ | ✅ | | ⚙️| 
| Plugin Wrapper | ❌ | ✅ | | | 
| Wrap Manifest | ⚙️ | ✅ | | | 
| **Uri Resolution** | ⚙️ | ✅ | | ⚙️ | 
| Uri: Filesystem|✅|✅| |
| Uri: IPFS |❌|✅| || |
| Uri: ENS |❌|✅| | | |

> TODO: Update table above according to test harness and maybe mention other wip clients (go, rust)

|status| |
| -- | -- |
|✅ | fully working|
|⚙️| partially working|
|❌|not yet implemented|

## Prerequisites

### Rust 

Proceed to installation by following [these instructions](https://www.rust-lang.org/tools/install).

>In MacOS, execute:
>```
>brew install rustup
>```

The installer will normally try to include Rust in your PATH variables. To make sure things are properlyu installed run: 
```
rustc --version
```
your output in this case should be something like `rustc 1.64.0 (a55dd71d5 2022-09-19)`. If the command can't be found, you might need to edit your PATH variables manually.


### Clone the repository
```
git clone https://github.com/polywrap/rust-client.git
```

# Installation and Build 

Make sure your `cwd` is the root folder of the repository, and build the package:

```
cargo build
```

# Test the client

Run this command from the root folder to execute all written tests
```
cargo bench
```
