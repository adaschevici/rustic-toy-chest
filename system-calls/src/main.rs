use nix::fcntl::{open, OFlag};
use nix::sys::signal::{signal, SigHandler, SIGINT};
use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};
use nix::sys::stat::Mode;
use nix::unistd::Whence;
use nix::unistd::{close, fork, lseek, pipe, read, write, ForkResult};
use std::fs::File;
use std::os::fd::AsRawFd;
use std::os::unix::io::{FromRawFd, RawFd};
use std::thread::sleep;
use std::time::Duration;

extern "C" fn sigint_handler(_: i32) {
    println!("Received SIGINT, exiting...");
    std::process::exit(0);
}

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
    // create a file
    let path = "tmp.txt";
    let buffer: &mut [u8] = &mut [0; 256];
    // open file
    let fd2: RawFd = open(
        path,
        OFlag::O_RDWR | OFlag::O_CREAT,
        Mode::S_IRUSR | Mode::S_IWUSR,
    )
    .expect("Failed to open file");

    // create buffer to write to file
    let nbytes = unsafe {
        write(File::from_raw_fd(fd2), b"Hello, nix!\n").expect("Failed to write to file")
    };
    let fd2: RawFd = open(
        path,
        OFlag::O_RDWR | OFlag::O_CREAT,
        Mode::S_IRUSR | Mode::S_IWUSR,
    )
    .expect("Failed to open file");
    let _ = lseek(fd2, 0, Whence::SeekSet).expect("Failed to seek to start of file");
    let _ = read(fd2, buffer).expect("Failed to read from file");
    close(fd2).expect("Failed to close file");
    println!("Read from file: {}", String::from_utf8_lossy(buffer));

    // add pipe functionality
    let (read_fd, write_fd) = pipe().expect("Failed to create pipe");
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let mut buffer: [u8; 128] = [0; 128];
            read(read_fd.as_raw_fd(), &mut buffer).expect("Failed to read from pipe");
            println!("Child read from pipe: {}", String::from_utf8_lossy(&buffer));
        }
        Ok(ForkResult::Parent { child, .. }) => {
            write(write_fd, b"Hello from parent").expect("Failed to write to pipe");
            println!("Hello from parent process. Child pid: {}", child);
        }
        Err(_) => {
            println!("Fork failed");
        }
    }

    // add a signal and handler
    unsafe {
        signal(SIGINT, SigHandler::Handler(sigint_handler)).expect("Failed to add signal handler");
    }
    loop {
        println!("Running...Press Ctrl+C to exit");
        sleep(Duration::from_secs(5));
    }
}
