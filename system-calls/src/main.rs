use nix::fcntl::{open, OFlag};
use nix::libc;
// use nix::mount::{mount, unmount, MsFlags};
use nix::poll::{poll, PollFd, PollFlags, PollTimeout};
use nix::sys::resource::{getrlimit, setrlimit, Resource};
use nix::sys::signal::{kill, signal, SigHandler, SIGINT, SIGTERM};
use nix::sys::socket::{socket, AddressFamily, SockFlag, SockType};
use nix::sys::stat::fchmod as chmod;
use nix::sys::stat::Mode;
use nix::unistd::gethostname;
use nix::unistd::Pid;
use nix::unistd::Whence;
use nix::unistd::{chdir, getcwd};
use nix::unistd::{close, fork, getgid, getuid, lseek, pipe, read, write, ForkResult};
use nix::unistd::{getpid, setsid};
use std::ffi::CString;
use std::ffi::OsString;
use std::fs::File;
use std::os::fd::{AsFd, AsRawFd};
use std::os::unix::io::{FromRawFd, RawFd};
use std::thread;
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
    // get user and group id
    let user_id = getuid();
    let group_id = getgid();
    println!("Current user ID: {}", user_id);
    println!("Current group ID: {}", group_id);
    // print out PATH env variable
    unsafe {
        let name = CString::new("PATH").expect("Failed to create CString");
        let value = libc::getenv(name.as_ptr());
        if !value.is_null() {
            let value_str = std::ffi::CStr::from_ptr(value)
                .to_string_lossy()
                .into_owned();
            println!("PATH: {}", value_str);
        } else {
            println!("PATH variable not found");
        }
    }
    // setting file permissions
    let path = "nix_tmp.txt";
    let fd2: RawFd = open(
        path,
        OFlag::O_RDWR | OFlag::O_CREAT,
        Mode::S_IRUSR | Mode::S_IWUSR,
    )
    .expect("Failed to open file");
    chmod(fd2, Mode::S_IRUSR | Mode::S_IWUSR).expect("Failed to change file permissions");
    println!("File permissions changed");
    close(fd2).expect("Failed to close file");
    // get hostname
    let hostname: OsString = gethostname().expect("Failed to get hostname");
    println!("Hostname: {}", hostname.to_string_lossy());
    // get and set resource limits
    let (soft_limit, hard_limit) =
        getrlimit(Resource::RLIMIT_NOFILE).expect("Failed to get resource limits");
    println!(
        "File descriptor limits: soft: {}, hard: {}",
        soft_limit, hard_limit
    );
    setrlimit(Resource::RLIMIT_NOFILE, 1024, hard_limit).expect("Failed to set resource limits");
    // change directory
    let initial_path = getcwd().expect("Failed to get current working directory");
    println!(
        "Initial working directory: {:?}",
        initial_path.to_string_lossy()
    );
    chdir("/tmp").expect("Failed to change directory");
    let new_path = getcwd().expect("Failed to get current working directory");
    println!("New working directory: {:?}", new_path.to_string_lossy());
    // process sessions starting
    println!("Current process ID: {:?}", getpid());
    // this will continue running so I am commenting it out for now
    //    match setsid() {
    //        Ok(session_id) => {
    //            println!("Started a new session with ID: {:?}", session_id);
    //        }
    //        Err(e) => {
    //            eprintln!("Failed to start a new session: {}", e);
    //        }
    //    }
    let (read_fd, write_fd) = pipe().expect("Failed to create pipe");
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        write(write_fd, b"Hello from awesome child").expect("Failed to write to pipe");
    });
    let mut fds = [PollFd::new(read_fd.as_fd(), PollFlags::POLLIN)];
    poll(&mut fds, PollTimeout::from(5000_u16)).expect("Failed to poll");
    if fds[0].revents().unwrap().contains(PollFlags::POLLIN) {
        let mut buffer: [u8; 128] = [0; 128];
        let _ = read(read_fd.as_raw_fd(), &mut buffer).expect("Failed to read from pipe");
        println!(
            "Parent read from pipe: {}",
            String::from_utf8_lossy(&buffer)
        );
    }
    // mount and unmount usb drive
    // not working on mac
    //     let source = Some("/dev/disk4");
    //     let target = "/mnt/usbdrive";
    //     let fstype = "exfat";
    //     let flags = MsFlags::empty();
    //     let data = None;
    //     mount(source, target, Some(fstype), flags, data).expect("Failed to mount usb drive");
    //     unmount(target, flags).expect("Failed to unmount usb drive");
    // Terminate process by pid
    //    let target_pid = Pid::from_raw(7518);
    //    kill(target_pid, SIGTERM).expect("Failed to terminate process");
    // add a signal and handler
    unsafe {
        signal(SIGINT, SigHandler::Handler(sigint_handler)).expect("Failed to add signal handler");
    }
    loop {
        println!("Running...Press Ctrl+C to exit");
        sleep(Duration::from_secs(5));
    }
}
