use clap::Parser;
use std::fs;
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

    /// use test input
    #[clap(default_value_t = false)]
    #[clap(short, long)]
    test: bool,
}

fn main() {
    let cli = Cli::parse();
    match &cli.day {
        1 => match cli.part_two {
            false => days::one::run(get_input(1, cli.test)),
            true => days::one::runtwo(get_input(1, cli.test)),
        },
        _ => print!("day not found"),
    }
}

fn get_input(day: u8, test: bool) -> String {
    let mut path = format!("input/input{}", day);
    if test {
        path = format!("{}test", path);
    }
    fs::read_to_string(path).expect("could not read input file")
}
