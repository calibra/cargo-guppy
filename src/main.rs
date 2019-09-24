use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Cargo.lock file analysis")]
struct Args {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "diff")]
    /// Perform a diff of two Cargo.lock files
    Diff {
        #[structopt(long)]
        json: bool,
        old: String,
        new: String,
    },
    #[structopt(name = "count")]
    /// Count the number of third-party deps (non-path)
    Count,
}

fn main() {
    let args = Args::from_args();

    let result = match args.cmd {
        Command::Diff { json, old, new } => cargo_guppy::cmd_diff(json, &old, &new),
        Command::Count => cargo_guppy::cmd_count(),
    };

    match result {
        Err(e) => println!("{}\nAborting...", e),
        Ok(()) => {}
    }
}
