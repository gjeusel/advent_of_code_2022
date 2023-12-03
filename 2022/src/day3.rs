// ________ Day 3: Rucksack Reorganization ________
// https://adventofcode.com/2022/day/3

use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use itertools::Itertools;

fn get_priority() -> HashMap<char, usize> {
    let lowercase_alphabet = "abcdefghijklmnopqrstuvwxyz";
    let upercase_alphabet = lowercase_alphabet.to_uppercase();
    let alphabet = format!("{}{}", lowercase_alphabet, upercase_alphabet);
    let iter_alphabet = alphabet.char_indices().map(|x| (x.1, x.0 + 1));

    let priority: HashMap<char, usize> = HashMap::from_iter(iter_alphabet);
    return priority;
}

fn day3_first_part(filepath: &Path) -> usize {
    let priority = get_priority();

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

    let count: usize = raw
        .into_iter()
        .filter(|x| x.len() == 1)
        .map(|x| priority.get(&x[0]).unwrap())
        .sum();

    return count;
}

fn day3_second_part(filepath: &Path) -> usize {
    let raw = fs::read_to_string(filepath).unwrap();
    let lines: Vec<&str> = raw.split("\n").filter(|&x| x.len() > 0).collect();

    let priority = get_priority();

    if lines.len() % 3 != 0 {
        panic!("{} lines can't be grouped 3 at a times.", lines.len())
    }

    let count: usize = lines
        .chunks(3)
        .map(|x| {
            let bag = x.join("");
            let set: HashSet<char> = x
                .iter()
                .map(|x| HashSet::from_iter(x.chars().into_iter()))
                .fold(HashSet::from_iter(bag.chars().into_iter()), |acc, x| {
                    &acc & &x
                });

            let total_win: usize = set.into_iter().map(|x| priority.get(&x).unwrap()).sum();
            return total_win;
        })
        .sum();

    return count;
}

pub fn run(datapath: &Path, part: u8) {
    let filepath = Path::join(datapath, Path::new("day3.txt"));
    println!("Using {}", filepath.as_os_str().to_str().unwrap());

    let count: usize;

    match part {
        1 => count = day3_first_part(filepath.as_path()),
        2 => count = day3_second_part(filepath.as_path()),
        _ => panic!("Day 3 does not have a {part} part."),
    }

    println!("Total priority found: {count}")
}
