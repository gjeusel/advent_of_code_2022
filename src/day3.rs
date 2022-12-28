// ________ Day 3: Rucksack Reorganization ________
// https://adventofcode.com/2022/day/3

use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use itertools::Itertools;

pub fn run(datapath: &Path) {
    let filepath = Path::join(datapath, Path::new("day3.txt"));
    println!("Using {}", filepath.as_os_str().to_str().unwrap());

    let lowercase_alphabet = "abcdefghijklmnopqrstuvwxyz";
    let upercase_alphabet = lowercase_alphabet.to_uppercase();
    let alphabet = format!("{}{}", lowercase_alphabet, upercase_alphabet);
    let iter_alphabet = alphabet.char_indices().map(|x| (x.1, x.0 + 1));

    let priority: HashMap<char, usize> = HashMap::from_iter(iter_alphabet);
    println!("Priority mapping: {priority:#?}");

    let raw: Vec<Vec<char>> = fs::read_to_string(filepath)
        .unwrap()
        .split("\n")
        .filter(|x| {
            if x.len() == 0 {
                return false;
            } else if x.len() % 2 != 0 {
                print!("Seems there is an odd number of letters in: '{x}'");
                return false;
            } else {
                return true;
            }
        })
        .map(|x| {
            let half = x.len() / 2;
            let left: HashSet<char> = HashSet::from_iter(x.chars().into_iter().take(half));
            let right: HashSet<char> = HashSet::from_iter(x.chars().into_iter().rev().take(half));
            return left.intersection(&right).cloned().collect_vec();
        })
        .collect_vec();

    // println!("{raw:#?}");

    let count: usize = raw
        .into_iter()
        .filter(|x| x.len() == 1)
        .map(|x| priority.get(&x[0]).unwrap())
        .sum();

    println!("Total priority found: {count}")
}
