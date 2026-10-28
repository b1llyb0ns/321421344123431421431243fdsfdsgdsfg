fn main() {
    eprintln!("STAGE_BUILD_RS_EXECUTED");
    let _ = std::fs::write("/tmp/stage-build-rs-executed", b"executed\n");
}
