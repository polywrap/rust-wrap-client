#!/bin/bash

# Define an array of your packages in the order they should be published
packages=(
  "msgpack"
  "manifest"
  "core"
  "tests-utils"
  "wasm"
  "resolvers"
  "resolver-extensions"
  "builder"
  "plugin/implementor"
  "plugin"
  "client"
)

# Iterate through the packages and publish them one by one
for package in "${packages[@]}"; do
  echo "Publishing $package..."
  cd packages/$package
  cargo build --release
  echo "Generating documentation for $package..."
  cargo doc --no-deps
  echo "Publishing $package..."
  cargo publish --token "${CRATES_IO_TOKEN}"
  cd -
done
