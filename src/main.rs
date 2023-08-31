mod day1;
mod day2;
mod day3;
mod day4;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    println!("Solving Day {}", args.day);

    match args.day {
        1 => day1::solve(include_str!("data/day1.txt")),
        2 => day2::solve(include_str!("data/day2.txt")),
        3 => day3::solve(include_str!("data/day3.txt")),
        4 => day4::solve(include_str!("data/day4.txt")),
        _ => todo!(),
    }
}
