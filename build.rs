use std::process::Command;

fn main() {
    let output = Command::new("javac")
        .args(["java_test/Test.java"])
        .output()
        .unwrap();

    if !output.status.success() {
        panic!("failed to compile java class");
    }
}
