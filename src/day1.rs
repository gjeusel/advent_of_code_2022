// ________ Day 1 - Calories ________
// https://adventofcode.com/2023/day/1

use std::{fs, path::Path};

pub fn run(datapath: &Path) {
    let filepath = Path::join(datapath, Path::new("day1.txt"));
    println!("In file {}", filepath.display());
}
