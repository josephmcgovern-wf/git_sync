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
    let mut output = Command::new("git").arg("pull").arg("upstream").arg(&branch).output().expect("failed to execute git pull");
    if !output.status.success() {
        output = Command::new("git").arg("pull").arg("origin").arg(&branch).output().expect("failed to execute git pull");
    }
    let mut to_print = output.stdout;
    if !output.status.success() {
        to_print = output.stderr;
    }
    println!("{}", String::from_utf8(to_print).expect("Expected stdout to be utf8"));
}
