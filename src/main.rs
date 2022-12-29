// ________ AOC - Advent Of Code 2022 ________
//
// https://adventofcode.com/2022

mod day1;
mod day2;
mod day3;
mod day4;

use clap::Parser;
use std::env;
use std::file;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author = "gjeusel", version, about = "Advent of Code baby")]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    // dbg!(args);

    let main_filepath = Path::join(env::current_dir().unwrap().as_path(), Path::new(file!()));
    let datapath = Path::join(
        main_filepath.parent().unwrap().parent().unwrap(),
        Path::new("data"),
    );

    match args.day {
        1 => day1::run(&datapath),
        2 => day2::run(&datapath, args.part),
        3 => day3::run(&datapath, args.part),
        4 => day4::run(&datapath, args.part),
        _ => println!("Day {} not yet implemented.", args.day),
    }
}
