for example in ./examples/src/*.rs;
do
    cd examples
    cargo run --example "$(basename "${example%.rs}")" --release
    if [ $? -ne 0 ]; then
        echo "Error running example: $(basename "${example%.rs}")"
        exit 1
    fi
    cd -
done