
// handle exe paths on windows & unix
#[cfg(windows)]
const MASTER_PROGRAM_PATH: &str = "target\\release\\master-program.exe";
#[cfg(unix)]
const MASTER_PROGRAM_PATH: &str = "./master-program";

fn main() {
    // become the master program using our stdin and stdout
    std::process::Command::new(MASTER_PROGRAM_PATH)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to start master program")
        .wait()
        .expect("Failed to wait for master program");
}
