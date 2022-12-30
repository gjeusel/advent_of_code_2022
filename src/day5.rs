use core::panic;
use std::{fs, path::Path};

use itertools::Itertools;
use regex::Regex;

struct Action {
    quantity: usize,
    from_col: usize,
    to_col: usize,
}

#[allow(dead_code)]
fn print_stacks(stacks: &Vec<Vec<char>>) {
    stacks.into_iter().enumerate().for_each(|(i, stack)| {
        println!("{} | {}", i + 1, stack.into_iter().join(" "));
    })
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
    let n_stacks: usize = raw_init_state
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
        .collect_vec()
        .into_iter()
        .rev()
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
                from_col: caps[2].parse::<usize>().unwrap() - 1,
                to_col: caps[3].parse::<usize>().unwrap() - 1,
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

    let (mut stacks, actions) = parse_file(&filepath);

    print_stacks(&stacks);

    actions.into_iter().for_each(|action| {
        let n = stacks[action.from_col].len();
        let split_at = n.saturating_sub(action.quantity);
        let head = stacks[action.from_col].split_off(split_at);
        stacks[action.to_col].extend(head.into_iter().rev());
    });

    let result = stacks
        .into_iter()
        .map(|stack| stack.last().expect("Stack is empty").clone())
        .join("");

    dbg!(&result);
}
