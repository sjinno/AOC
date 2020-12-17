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
        if path[position] == '#' {
            tree_count += 1;
        }
    }
    println!("Answer for part 1: {}.", tree_count);
    Ok(())
}

pub fn part_2() -> std::io::Result<()> {
    let filename = "files/day3.txt";
    // let filename = "files/day3_example.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]; // (rise, run)

    let mut data: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        data.push(line.unwrap().chars().collect());
    }

    let length = data[0].len();
    let mut counts = vec![];
    for slope in slopes {
        let mut tree_count = 0;
        let mut position = 0;
        let mut skip_line = 0;
        for row in 1..data.len() {
            skip_line += 1;
            if skip_line != slope.0 {
                continue;
            }
            skip_line = 0;
            position += slope.1;
            if position > length - 1 {
                position = position.rem_euclid(length);
            }
            if data[row][position] == '#' {
                tree_count += 1;
            }
        }
        println!(
            "Tree count for slopw {}/{}: {}.",
            slope.0, slope.1, tree_count
        );
        counts.push(tree_count);
    }

    let product: i64 = counts.iter().fold(1, |acc, x| acc * x);
    println!("Product of tree counts: {:?}.", product);
    Ok(())
}
