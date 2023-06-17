# Native Polywrap Client

This package implements [uniffi](https://github.com/mozilla/uniffi-rs) for Polywrap Client. It aims to easily
provide a way to generate a dynamic library that can be imported into other languages.

## Usage

### Swift

- Install [swiftformat](https://github.com/nicklockwood/SwiftFormat)
```bash
$ brew install swiftformat
```
Go to `scripts/build_swift_framework.sh` and change:
- `PATH`: Set this with the path of your cargo binary
- `RUST_PROJ`: Set this to your rust client local path
- `IOS_PROJ`: Set this to your swift client local path

Once you have done that, run 
```bash
$ ./scripts/build_swift_framework.sh
```
and it will generate a dynamic library inside of the swift project (With the path given in the variable `IOS_PROJ`). Now you should be able to build locally the Polywrap Client for iOS.
```bash
$ swift build
```