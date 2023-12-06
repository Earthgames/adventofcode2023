use clap::Parser;
use std::fs;
mod days;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// The day
    #[clap(default_value_t = 6)]
    #[arg(short, long)]
    day: u8,

    /// part two
    #[clap(default_value_t = true)]
    #[clap(short, long)]
    part_two: bool,

    /// use test input
    #[clap(default_value_t = true)]
    #[clap(short, long)]
    test: bool,
}

fn main() {
    let cli = Cli::parse();
    match &cli.day {
        1 => match cli.part_two {
            false => days::one::run(get_input(cli.day, cli.test)),
            true => days::one::runtwo(get_input(cli.day, cli.test)),
        },
        2 => match cli.part_two {
            false => days::two::run(get_input(cli.day, cli.test)),
            true => days::two::runtwo(get_input(cli.day, cli.test)),
        },
        3 => match cli.part_two {
            false => days::three::run(get_input(cli.day, cli.test)),
            true => days::three::runtwo(get_input(cli.day, cli.test)),
        },
        4 => match cli.part_two {
            false => days::four::run(get_input(cli.day, cli.test)),
            true => days::four::runtwo(get_input(cli.day, cli.test)),
        },
        5 => match cli.part_two {
            false => days::five::run(get_input(cli.day, cli.test)),
            true => days::five::runtwo(get_input(cli.day, cli.test)),
        },
        6 => match cli.part_two {
            false => days::six::run(get_input(cli.day, cli.test)),
            true => days::six::runtwo(get_input(cli.day, cli.test)), //TODO change
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
