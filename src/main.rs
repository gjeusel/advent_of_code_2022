use clap::Parser;
use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::file;
use std::fs;
use std::path::Path;
use std::vec;

// ________ Day 1 - Calories ________
// https://adventofcode.com/2022/day/1

fn day1(datapath: &Path) {
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

// ________ Day 2 - Chifumi ________
// https://adventofcode.com/2022/day/2

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Outcome {
    Win,
    Draw,
    Loose,
}

struct Points {
    outcome: HashMap<Outcome, u32>,
    action: HashMap<Action, u32>,
}

fn define_chifumi_points() -> Points {
    return Points {
        //
        // 0 if you lost, 3 if the round was a draw, and 6 if you won
        outcome: HashMap::from([(Outcome::Win, 6), (Outcome::Draw, 3), (Outcome::Loose, 0)]),
        //
        // 1 for Rock, 2 for Paper, and 3 for Scissor
        action: HashMap::from([(Action::Rock, 1), (Action::Paper, 2), (Action::Scissors, 3)]),
    };
}

fn day2_first_part(filepath: &Path) -> (u32, u32) {
    // A for Rock, B for Paper, and C for Scissors
    // X for Rock, Y for Paper, and Z for Scissors
    let map_actions = HashMap::from([
        ("A", Action::Rock),
        ("B", Action::Paper),
        ("C", Action::Scissors),
        ("X", Action::Rock),
        ("Y", Action::Paper),
        ("Z", Action::Scissors),
    ]);

    let strategy: Vec<(&Action, &Action)> = fs::read_to_string(filepath)
        .unwrap()
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| map_actions.get(x))
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let points = define_chifumi_points();
    let mut score_elf: u32 = 0;
    let mut score_me: u32 = 0;

    for i in 0..strategy.len() {
        let round = strategy[i];
        let action_elf = round.0;
        let action_me = round.1;

        score_elf += points.action[action_elf];
        score_me += points.action[action_me];

        if action_elf == action_me {
            let outcome = &Outcome::Draw;
            score_elf += points.outcome[outcome];
            score_me += points.outcome[outcome];
        } else {
            let score_win = points.outcome[&Outcome::Win];
            match action_elf {
                Action::Rock => match action_me {
                    Action::Scissors => score_elf += score_win,
                    Action::Paper => score_me += score_win,
                    _ => {}
                },
                Action::Paper => match action_me {
                    Action::Rock => score_elf += score_win,
                    Action::Scissors => score_me += score_win,
                    _ => {}
                },
                Action::Scissors => match action_me {
                    Action::Paper => score_elf += score_win,
                    Action::Rock => score_me += score_win,
                    _ => {}
                },
            }
        }
    }

    return (score_elf, score_me);
}

fn day2_second_part(filepath: &Path) -> (u32, u32) {
    let map_actions = HashMap::from([
        ("A", Action::Rock),
        ("B", Action::Paper),
        ("C", Action::Scissors),
    ]);

    // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
    let map_outcome = HashMap::from([
        ("X", Outcome::Loose),
        ("Y", Outcome::Draw),
        ("Z", Outcome::Win),
    ]);

    // A Y
    // B X
    // C Z

    let strategy: Vec<(&Action, &Outcome)> = fs::read_to_string(filepath)
        .unwrap()
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|line| {
            let arr: (&str, &str) = line.split(" ").collect_tuple().unwrap();
            return (
                map_actions.get(arr.0).unwrap(),
                map_outcome.get(arr.1).unwrap(),
            );
        })
        .collect();

    let points = define_chifumi_points();
    let mut score_elf = 0;
    let mut score_me = 0;

    for i in 0..strategy.len() {
        let round = strategy[i];
        let action_elf = round.0;
        let outcome = round.1;

        let action_me: Action;

        match outcome {
            Outcome::Win => {
                score_me += points.outcome[&Outcome::Win];
                match action_elf {
                    Action::Rock => action_me = Action::Paper,
                    Action::Paper => action_me = Action::Scissors,
                    Action::Scissors => action_me = Action::Rock,
                }
            }
            Outcome::Draw => {
                score_me += points.outcome[&outcome];
                score_elf += points.outcome[&outcome];
                action_me = action_elf.clone();
            }
            Outcome::Loose => {
                score_elf += points.outcome[&Outcome::Win];
                match action_elf {
                    Action::Rock => action_me = Action::Scissors,
                    Action::Paper => action_me = Action::Rock,
                    Action::Scissors => action_me = Action::Paper,
                }
            }
        }

        score_elf += points.action[action_elf];
        score_me += points.action[&action_me];
    }

    return (score_elf, score_me);
}

fn day2(datapath: &Path, part: u8) {
    let filepath = Path::join(datapath, Path::new("day2.txt"));

    let score_elf: u32;
    let score_me: u32;

    match part {
        1 => (score_elf, score_me) = day2_first_part(filepath.as_path()),
        2 => (score_elf, score_me) = day2_second_part(filepath.as_path()),
        _ => panic!("Day 2 does not have a {part} part."),
    }

    if score_elf > score_me {
        println!("The elf win with {score_elf} points (versus me: {score_me})")
    } else {
        println!("I won, with {score_me} and the elf a score of {score_elf}")
    }
}

// ________ Main ________

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
        1 => day1(&datapath),
        2 => day2(&datapath, args.part),
        _ => println!("Day {} not yet implemented.", args.day),
    }
}
