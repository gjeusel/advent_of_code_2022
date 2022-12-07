use std::env;
use std::file;
use std::fs;
use std::path::Path;
use std::vec;

fn day1(datapath: &Path) {
    // let filepath = datapath
    let filepath = Path::join(datapath, Path::new("day1/input.txt"));
    println!("In file {}", filepath.display());

    let contents = fs::read_to_string(filepath).expect("failed to read day1 input file");
    println!("With text:\n{contents}");

    let mut vectors: Vec<Vec<u8>> = vec![];

    let mut i_vector: usize = 0;
    for line in contents.split_whitespace() {
        if line == "\n" {
            i_vector += 1;
            println!();
        } else {
            let n = line.parse::<u8>().expect("Failed to parse line");
            vectors[i_vector].push(n);
        }
    }
}

fn main() {
    let main_filepath = Path::join(env::current_dir().unwrap().as_path(), Path::new(file!()));
    let datapath = Path::join(
        main_filepath.parent().unwrap().parent().unwrap(),
        Path::new("data"),
    );

    // println!("main_filepath: {}", main_filepath.display());

    day1(&datapath)
}
