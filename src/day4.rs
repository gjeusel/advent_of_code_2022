use std::{fs, ops::RangeInclusive, path::Path};

use itertools::Itertools;
use range_set::RangeIntersect;

type RangeTuple = (RangeInclusive<usize>, RangeInclusive<usize>);

fn parse_file(filepath: &Path) -> Vec<RangeTuple> {
    let raw = fs::read_to_string(filepath).unwrap();
    let lines: Vec<&str> = raw.split("\n").filter(|&x| x.len() > 0).collect();

    return lines
        .iter()
        .map(|&x| {
            return x
                .split(",")
                .map(|x| {
                    let (start, end) = x
                        .split("-")
                        .map(|x| x.parse().expect(format!("Failed to parse {x}").as_str()))
                        .collect_tuple()
                        .unwrap();
                    return RangeInclusive::new(start, end);
                })
                .collect_tuple()
                .unwrap();
        })
        .collect_vec();
}

fn day4_first_part(filepath: &Path) -> usize {
    let ranges = parse_file(filepath);

    let count = ranges
        .iter()
        // .map(|x| {
        //     let inter = RangeIntersect::compare(&x.0, &x.1);
        //     println!("Intersect of {x:?} is {inter:?}");
        //     return x;
        // })
        .filter(|x| match RangeIntersect::compare(&x.0, &x.1) {
            None => false,
            Some(value) => {
                ![RangeIntersect::OverlapsLeft, RangeIntersect::OverlapsRight].contains(&value)
            }
        })
        // .map(|x| dbg!(x))
        .count();

    return count;
}

fn day4_second_part(filepath: &Path) -> usize {
    let ranges = parse_file(filepath);

    let count = ranges
        .iter()
        .filter(|x| match RangeIntersect::compare(&x.0, &x.1) {
            None => false,
            _ => true,
        })
        .count();

    return count;
}

pub fn run(datapath: &Path, part: u8) {
    let filepath = Path::join(datapath, Path::new("day4.txt"));
    println!("Using {}", filepath.as_os_str().to_str().unwrap());

    let count: usize;

    match part {
        1 => count = day4_first_part(filepath.as_path()),
        2 => count = day4_second_part(filepath.as_path()),
        _ => panic!("Day 4 does not have a {part} part."),
    }

    println!("Total found: {count}")
}
