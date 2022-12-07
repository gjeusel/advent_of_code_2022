use std::env;
use std::file;
use std::fs;
use std::path::Path;
use std::vec;

fn day1(datapath: &Path) {
    // https://adventofcode.com/2022/day/1

    let filepath = Path::join(datapath, Path::new("day1/input.txt"));
    println!("In file {}", filepath.display());

    let raw_inventories = fs::read_to_string(filepath).expect("failed to read day1 input file");

    let mut inventories: Vec<Vec<u32>> = vec![vec![]];

    let mut i: usize = 0;
    for line in raw_inventories.split("\n") {
        if line.is_empty() {
            i += 1;
            inventories.push(vec![])
        } else {
            let item_calory = line
                .parse::<u32>()
                .expect(&format!("Failed to parse line: '{}'", line.to_string()));
            inventories[i].push(item_calory);
        }
    }

    let mut calory_per_inventory: Vec<u32> = vec![];
    for inventory in inventories {
        calory_per_inventory.push(inventory.iter().sum())
    }

    let max_calories = calory_per_inventory.iter().max().expect("Failed to sum");
    println!("The calories of the highest inventory is: {}", max_calories)
}

fn main() {
    let main_filepath = Path::join(env::current_dir().unwrap().as_path(), Path::new(file!()));
    let datapath = Path::join(
        main_filepath.parent().unwrap().parent().unwrap(),
        Path::new("data"),
    );

    // Day 1 Challenge
    day1(&datapath)
}
