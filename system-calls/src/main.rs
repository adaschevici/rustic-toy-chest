use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};
use nix::unistd::{fork, ForkResult};

fn main() {
    // TODO: Add error handling and a CLI interface using inquirer-rs and clap
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("Hello from parent process. Child pid: {}", child);
        }
        Ok(ForkResult::Child) => {
            println!("Hello from child process");
        }
        Err(_) => {
            println!("Fork failed");
        }
    }

    // Create a socket
    let fd = socket(
        AddressFamily::Inet,
        SockType::Stream,
        SockFlag::empty(),
        None,
    )
    .expect("Failed to create socket");
}
