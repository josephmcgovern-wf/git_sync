use structopt::StructOpt;
use std::process::Command;

#[derive(StructOpt)]
struct Cli {
    /// What to name the new branch
    #[structopt(short = "b", long = "branch")]
    branch: Option<String>,
}

fn main() {
    let args = Cli::from_args();
    let branch = match args.branch {
        Some(b) => b,
        None => String::from("master")
    };
    let mut output = Command::new("git").arg("-c").arg("color.pull=always").arg("pull").arg("upstream").arg(&branch).output().expect("failed to execute git pull");
    if !output.status.success() {
        output = Command::new("git").arg("-c").arg("color.pull=always").arg("pull").arg("origin").arg(&branch).output().expect("failed to execute git pull");
    }
    let mut message = output.stdout;
    if message.len() == 0 {
        message = output.stderr;
    }
    let result = String::from_utf8_lossy(&message).into_owned();
    print!("{}", result);
}
