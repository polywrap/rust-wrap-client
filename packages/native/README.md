# FFI C Binding of Polywrap Client

This package contains the necessary C bindings to execute Polywrap client from any
language that can import dynamic libraries (`.so` in linux or `.dylib` in mac).

In order to import it in other project (like c++ or swift), you need to:

- Build this package using `cargo build --release --lib`
- Run `cbindgen --config cbindgen.toml --crate polywrap_ffi_c --output header.h --lang c`

This will generate a `headers.hpp` file in the current crate & generate the `target` folder.
In the `target/release` folder (which is in the root of the monorepo) is where the `.so` or `.dylib`
file will be generated. Then this dynamic library can be imported, along with the header file.

For implementation examples on how it can be used:
- Swift: https://github.com/cbrzn/swift-client
- C++: https://github.com/cbrzn/c-playground