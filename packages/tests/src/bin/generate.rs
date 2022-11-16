use std::process::Command;
use std::fs;

fn main() {
    let mut clone = Command::new("git");

    clone.arg("clone").arg("git@github.com:polywrap/wasm-test-harness.git");
    clone.output().expect("Clone failed");

    let mut checkout = Command::new("git");
    checkout.current_dir("./wasm-test-harness");
    checkout.arg("checkout").arg("tags/v0.1.1");

    checkout.output().expect("failed checkout");

    let mut move_wrappers = Command::new("mv");
    move_wrappers.current_dir("./wasm-test-harness");
    move_wrappers.arg("./wrappers").arg("../cases")
        .output().expect("Move failed");

    fs::remove_dir_all("wasm-test-harness").expect("Remove cloned dir failed");
}