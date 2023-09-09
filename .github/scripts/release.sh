#!/bin/bash

# Exit script if any command fails
set -e

# Define arrays of your packages in the order they should be published
core_packages=(
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
)

external_packages=(
  "plugins/http"
  "plugins/fs"
  "plugins/ethereum-wallet"
  "plugins/logger"
  "default-config"
  "polywrap"
)

# Depending on the argument, decide which array to use
if [ "$1" == "core" ]; then
    packages=("${core_packages[@]}")
elif [ "$1" == "external" ]; then
    packages=("${external_packages[@]}")
else
    echo "Invalid argument. Use 'core' or 'external'."
    exit 1
fi

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
