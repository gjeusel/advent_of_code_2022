// ________ Day 1 - Calories ________
// https://adventofcode.com/2022/day/1

use std::{fs, path::Path};

pub fn run(datapath: &Path) {
    let filepath = Path::join(datapath, Path::new("day1.txt"));
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
