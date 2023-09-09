## Contributing

Thank you for considering contributing to the Polywrap Rust client!

Contributions are more than welcome. If you find a bug or have suggestions for improvements, please open an issue. And if you'd like to contribute code, we would be happy to review a pull request, also, feel free to look through this repository's issues to see what we're focused on solving.


## Pre-Requisites

You must have rust installed on your machine. If you don't, you can install it [here](https://www.rust-lang.org/tools/install). Make sure you're using stable version and not nightly

## Build and test

Before running tests, cases need to be generated. To do so, run:
```shell
$ cargo run --package polywrap_tests_utils --bin generate
```
Now you will be able to run tests of all crates:
```shell
$ cargo test --release
```

## Feedback and discussions

For questions, suggestions, or discussions, open an issue or create a discussion within the [Polywrap Discord](https://discord.polywrap.io).

Happy coding!
