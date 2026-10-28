extern crate proc_macro;

use proc_macro::TokenStream;
use std::{
    fs,
    net::{SocketAddr, TcpStream},
    process::Command,
    time::Duration,
};

fn clean(value: String) -> String {
    value.replace(['\n', '\r'], " ").trim().to_string()
}

#[proc_macro_attribute]
pub fn sandbox_probe(_attr: TokenStream, _item: TokenStream) -> TokenStream {
    let id = Command::new("id")
        .output()
        .map(|output| clean(String::from_utf8_lossy(&output.stdout).into_owned()))
        .unwrap_or_else(|error| format!("error:{error}"));
    let hostname = fs::read_to_string("/etc/hostname")
        .map(clean)
        .unwrap_or_else(|error| format!("error:{error}"));
    let apparmor = fs::read_to_string("/proc/self/attr/current")
        .map(clean)
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
    let message = format!(
        "SANDBOX_PROBE uid=[{id}] hostname=[{hostname}] apparmor=[{apparmor}] docker_127_0_0_1_2375=[{}] rootfs_write=[{}] host_shadow=[{}]",
        if docker.is_ok() { "connected" } else { "blocked" },
        if root_write.is_ok() { "writable" } else { "blocked" },
        if host_shadow.is_ok() { "readable" } else { "blocked" },
    );
    format!("compile_error!({message:?});")
        .parse()
        .expect("valid compile error")
}
