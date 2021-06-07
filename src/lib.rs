use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener};
use std::os::unix::prelude::*;
use std::thread;

pub fn debug_init(bind_addr: SocketAddr) {
    println!("Binding to {}", bind_addr);
    let listener = TcpListener::bind(bind_addr).unwrap();
    println!("Connecting...");
    let (mut peer, _) = listener.accept().unwrap();
    println!("Connected!");

    let mut logpipe: [RawFd; 2] = Default::default();
    let (stdout, stderr) = unsafe {
        let stdout: RawFd = libc::dup(libc::STDOUT_FILENO);
        if stdout < 0 {
            panic!();
        }
        let stderr: RawFd = libc::dup(libc::STDERR_FILENO);
        if stderr < 0 {
            panic!();
        }
        libc::pipe(logpipe.as_mut_ptr());
        libc::dup2(logpipe[1], libc::STDOUT_FILENO);
        libc::dup2(logpipe[1], libc::STDERR_FILENO);
        (stdout, stderr)
    };

    thread::spawn(move || {
        let file = unsafe { File::from_raw_fd(logpipe[0]) };
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        loop {
            buffer.clear();
            if let Ok(len) = reader.read_line(&mut buffer) {
                if len == 0 || peer.write(buffer.as_bytes()).is_err() {
                    break;
                }
            }
        }
        unsafe {
            libc::dup2(stdout, libc::STDOUT_FILENO);
            libc::dup2(stderr, libc::STDERR_FILENO);
        }
        println!("Stdout pipe broken! Reverted back.")
    });
}
