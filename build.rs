use std::process::Command;

fn main() {
    let status = Command::new("just")
        .status()
        .expect("Failed to execute just command");

    if !status.success() {
        panic!("Failed to run 'just' command");
    }
} 