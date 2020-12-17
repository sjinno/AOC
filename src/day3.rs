use std::fs::File;
use std::io::{BufRead, BufReader, Read};

// #[allow(unused_variables, unused_mut)]
pub fn part_1() -> std::io::Result<()> {
    let filename = "files/day3.txt";
    // let filename = "files/day3_example.txt";
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);

    let mut tree_count = 0;
    let length = reader.by_ref().lines().next().unwrap().unwrap().len();
    let mut position = 0;
    for line in reader.lines() {
        position += 3;
        if position > length - 1 {
            position = position.rem_euclid(length);
        }
        let path: Vec<char> = line.unwrap().chars().collect();
        // println!("{}", path[position]);
        if path[position] == '#' {
            tree_count += 1;
        }
    }
    println!("Answer for part 1: {}.", tree_count);
    Ok(())
}
