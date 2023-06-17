set -e # Helps to give error info

# Project paths
RUST_PROJ="/Users/cesar/dev/polywrap/rust-client/packages/native"
IOS_PROJ="/Users/cesar/dev/polywrap/swift/PolywrapClient"

LOCAL_UDL="src/polywrap_native.udl"
UDL_NAME="polywrap_native"
FRAMEWORK_NAME="PolywrapClientNative"
SWIFT_INTERFACE="PolywrapClientLib"

# Binary paths
PATH="$PATH:/Users/cesar/.cargo/bin" # Adds the rust compiler

cd "$RUST_PROJ"

# Compile the rust
cargo build --target aarch64-apple-ios
cargo build --target aarch64-apple-ios-sim
cargo build --target x86_64-apple-ios

# Remove old files if they exist
IOS_ARM64_FRAMEWORK="$FRAMEWORK_NAME.xcframework/ios-arm64/$FRAMEWORK_NAME.framework"
IOS_SIM_FRAMEWORK="$FRAMEWORK_NAME.xcframework/ios-arm64_x86_64-simulator/$FRAMEWORK_NAME.framework"

rm -f "$IOS_ARM64_FRAMEWORK/$FRAMEWORK_NAME"
rm -f "$IOS_ARM64_FRAMEWORK/Headers/${UDL_NAME}FFI.h"
rm -f "$IOS_SIM_FRAMEWORK/$FRAMEWORK_NAME"
rm -f "$IOS_SIM_FRAMEWORK/Headers/${UDL_NAME}FFI.h"

rm -f ../../target/universal.a
rm -rf include/ios/*

# Make dirs if it doesn't exist
mkdir -p include/ios

# UniFfi bindgen
cargo run --bin uniffi-bindgen generate "$LOCAL_UDL" --language swift --out-dir ./include/ios

# Make fat lib for sims
lipo -create \
    "../../target/aarch64-apple-ios-sim/debug/lib${UDL_NAME}.a" \
    "../../target/x86_64-apple-ios/debug/lib${UDL_NAME}.a" \
    -output ../../target/universal.a

# Move binaries
cp "../../target/aarch64-apple-ios/debug/lib${UDL_NAME}.a" \
    "$IOS_ARM64_FRAMEWORK/$FRAMEWORK_NAME.a"
cp ../../target/universal.a \
    "$IOS_SIM_FRAMEWORK/$FRAMEWORK_NAME.a"

# Move headers
cp "include/ios/${UDL_NAME}FFI.h" \
    "$IOS_ARM64_FRAMEWORK/Headers/${UDL_NAME}FFI.h"
cp "include/ios/${UDL_NAME}FFI.h" \
    "$IOS_SIM_FRAMEWORK/Headers/${UDL_NAME}FFI.h"

# Move swift interface
sed "s/${UDL_NAME}FFI/$FRAMEWORK_NAME/g" "include/ios/$UDL_NAME.swift" > "include/ios/$SWIFT_INTERFACE.swift"

cp -r "include/ios/" "$IOS_PROJ/Sources/PolywrapClient/include"
rm "$IOS_PROJ/Sources/PolywrapClient/include/$UDL_NAME.swift"
cp "$IOS_ARM64_FRAMEWORK/$FRAMEWORK_NAME.a" "$IOS_PROJ/Sources/PolywrapClient/Frameworks/$FRAMEWORK_NAME.xcframework/ios-arm64/$FRAMEWORK_NAME.a"
cp "$IOS_SIM_FRAMEWORK/$FRAMEWORK_NAME.a" "$IOS_PROJ/Sources/PolywrapClient/Frameworks/$FRAMEWORK_NAME.xcframework/ios-arm64_x86_64-simulator/$FRAMEWORK_NAME.a"