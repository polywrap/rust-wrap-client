#!/bin/bash

# Exit script if any command fails
set -e

# Define an array of your packages in the order they should be published
packages=(
  "manifest"
  "uri"
  "core/macros"
  "core"
  "plugin/implementor"
  "plugin"
  "tests-utils"
  "wasm"
  "resolvers"
  "resolver-extensions"
  "builder"
  "client"
  "plugins/http"
  "plugins/fs"
  "plugins/ethereum-wallet"
  "default-config"
  "polywrap"
)

# Iterate through the packages and publish them one by one
for package in "${packages[@]}"; do
  echo "Publishing $package..."
  cd packages/$package
  echo "Generating documentation for $package..."
  cargo doc --no-deps
  echo "Publishing $package..."
  cargo publish --token "${CRATES_IO_TOKEN}"
  rm -rf target/
  cd -
done
