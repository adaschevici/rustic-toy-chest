use dotenvy::dotenv;
use std::env;
use std::process::Command;
use std::process::ExitStatus;

fn main() {
    dotenv().ok();

    let mode = match env::var("MODE") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("MODE is not set");
            return;
        }
    };

    // Print a message to stderr
    Command::new("echo")
        .arg("setup_env.sh: script started")
        .status()
        .expect("failed to run echo");

    let status: ExitStatus;

    if mode == "dev" {
        status = Command::new("bash")
            .arg("scripts/setup_env.sh")
            .status()
            .expect("Failed to execute Bash script");

        // Check if the script executed successfully
        if !status.success() {
            panic!("Failed to run setup_env.sh");
        }

        if status.success() {
            eprintln!("retrieved db url");
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
}
