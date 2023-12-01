use clap::Parser;
mod days;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// The day
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

fn main() {
    let cli = Cli::parse();
    match &cli.day {
        1 => days::one::run(),
        _ => print!("day not found"),
    }
}
