use core::panic;
use std::{fs, path::Path};

use itertools::Itertools;
use regex::Regex;

struct Action {
    quantity: u8,
    from_col: u8,
    to_col: u8,
}

fn parse_file(filepath: &Path) -> (Vec<Vec<char>>, Vec<Action>) {
    let raw = fs::read_to_string(filepath).unwrap();
    let (raw_init_state, raw_actions) = raw
        .split("\n\n")
        .collect_tuple()
        .expect("Failed to split raw file between initial state and actions");

    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    let mut n_stacks: usize = raw_init_state
        .split("\n")
        .last()
        .expect(format!("Failed to get last line of {raw_init_state}").as_str())
        .split_whitespace()
        .map(|x| {
            x.parse()
                .expect(format!("Failed to parse {x} into a u8").as_str())
        })
        .max()
        .unwrap();

    let mut stacks = vec![Vec::<char>::new(); n_stacks];

    raw_init_state
        .split("\n")
        .filter(|&line| line.starts_with("["))
        .for_each(|line| {
            // dbg!(line);
            line.chars()
                .chunks(4)
                .into_iter()
                .enumerate()
                .into_iter()
                .for_each(|(i, chars)| {
                    let s: String = chars.into_iter().collect();
                    // dbg!(&s);
                    if s.trim().len() != 0 {
                        stacks[i].push(s.chars().nth(1).unwrap());
                    }
                });
        });

    // move 1 from 2 to 1
    // move 3 from 1 to 3
    // move 2 from 2 to 1
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let actions = raw_actions
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|line| {
            let caps = re
                .captures(line)
                .expect(format!("Failed to parse '{line}'").as_str());
            if caps.len() != 4 {
                panic!("Failed to extract the right number of captures on '{line}'");
            }
            return Action {
                quantity: caps[1].parse().unwrap(),
                from_col: caps[2].parse().unwrap(),
                to_col: caps[3].parse().unwrap(),
            };
        })
        .collect_vec();

    return (stacks, actions);
}

pub fn run(datapath: &Path, part: u8) {
    let filepath = Path::join(datapath, Path::new("day5.txt"));
    println!(
        "Using {} for part={part}",
        filepath.as_os_str().to_str().unwrap()
    );

    let (stacks, actions) = parse_file(&filepath);

    println!("stacks=\n{stacks:?}")

    // actions.into_iter().map(|action| {

    // })
}
