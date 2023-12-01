use clap::Parser;
mod days;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// The day
    // #[clap(default_value_t = 1)]
    #[arg(short, long)]
    day: u8,

    /// part two
    #[clap(default_value_t = false)]
    #[clap(short, long)]
    part_two: bool,
}

fn main() {
    let cli = Cli::parse();
    match &cli.day {
        1 => match cli.part_two {
            false => days::one::run(),
            true => days::one::runtwo(),
        },
        _ => print!("day not found"),
    }
}
