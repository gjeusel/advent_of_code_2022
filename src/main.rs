use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::file;
use std::fs;
use std::path::Path;
use std::vec;

#[allow(dead_code)]
fn day1(datapath: &Path) {
    // https://adventofcode.com/2022/day/1

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

#[allow(dead_code)]
fn day2(datapath: &Path) {
    // https://adventofcode.com/2022/day/2

    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Moves {
        Rock,
        Paper,
        Scissors,
    }

    // A for Rock, B for Paper, and C for Scissors
    // X for Rock, Y for Paper, and Z for Scissors
    let map_moves = HashMap::from([
        ("A", Moves::Rock),
        ("B", Moves::Paper),
        ("C", Moves::Scissors),
        ("X", Moves::Rock),
        ("Y", Moves::Paper),
        ("Z", Moves::Scissors),
    ]);

    let filepath = Path::join(datapath, Path::new("day2.txt"));

    let strategy: Vec<(&Moves, &Moves)> = fs::read_to_string(filepath)
        .unwrap()
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| map_moves.get(x))
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let map_score = HashMap::from([
        // 1 for Rock, 2 for Paper, and 3 for Scissor
        (Moves::Rock, 1),
        (Moves::Paper, 2),
        (Moves::Scissors, 3),
    ]);

    let mut score_elf = 0;
    let mut score_me = 0;

    for i in 0..strategy.len() {
        let round = strategy[i];
        let move_elf = round.0;
        let move_me = round.1;

        score_elf += map_score[move_elf];
        score_me += map_score[move_me];

        // 0 if you lost, 3 if the round was a draw, and 6 if you won
        let score_draw = 3;
        let score_win = 6;

        if move_elf == move_me {
            score_elf += score_draw;
            score_me += score_draw;
        } else {
            match move_elf {
                Moves::Rock => match move_me {
                    Moves::Scissors => score_elf += score_win,
                    Moves::Paper => score_me += score_win,
                    _ => {}
                },
                Moves::Paper => match move_me {
                    Moves::Rock => score_elf += score_win,
                    Moves::Scissors => score_me += score_win,
                    _ => {}
                },
                Moves::Scissors => match move_me {
                    Moves::Paper => score_elf += score_win,
                    Moves::Rock => score_me += score_win,
                    _ => {}
                },
            }
        }
    }

    if score_elf > score_me {
        println!("The elf win with {score_elf} points (versus me: {score_me})")
    } else {
        println!("I won, with {score_me} and the elf a score of {score_elf}")
    }
}

fn main() {
    let main_filepath = Path::join(env::current_dir().unwrap().as_path(), Path::new(file!()));
    let datapath = Path::join(
        main_filepath.parent().unwrap().parent().unwrap(),
        Path::new("data"),
    );

    // day1(&datapath)

    day2(&datapath)
}
