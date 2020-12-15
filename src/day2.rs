// --- Day 2: Password Philosophy ---

// Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

// The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

// Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

// To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

// For example, suppose you have the following list:

// 1-3 a: abcde
// 1-3 b: cdefg
// 2-9 c: ccccccccc

// Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

// In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn password_philosophy() -> std::io::Result<()> {
    let filename = "files/passwords.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let data = line.unwrap();
        let split_data: Vec<&str> = data.split(' ').collect();
        
        if split_data.len() == 1 {
            break;
        }

        let range: Vec<&str> = split_data[0].split('-').collect();
        let lower_bound = range[0].parse::<i32>().unwrap() - 1;
        let upper_bound = range[1].parse::<i32>().unwrap() + 1;
        let target_char = split_data[1].chars().next().unwrap();
        let password = split_data[2];
        
        let mut char_count = 0;
        for c in password.chars() {
            if c == target_char {
                char_count += 1;
            }
        }

        if char_count > lower_bound && char_count < upper_bound {
            count += 1;
        }
    }

    println!("Answer for part 1: {}.", count); // Print the number of valid passwords.
    Ok(())
}

pub fn part_2() -> std::io::Result<()> {
    let filename = "files/passwords.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut count = 0;
    for line in reader.lines() {
        let data = line.unwrap();
        let split_data: Vec<&str> = data.split(' ').collect();
        
        if split_data.len() == 1 {
            break;
        }

        let range: Vec<&str> = split_data[0].split('-').collect();
        let idx_of_first_char = range[0].parse::<i32>().unwrap() - 1;
        let idx_of_second_char = range[1].parse::<i32>().unwrap() - 1;
        let target_char = split_data[1].chars().next().unwrap();
        let password = split_data[2];

        if validate_password(target_char, password, idx_of_first_char, idx_of_second_char) {
            count += 1;
        }
    }

    println!("Answer for part 2: {}.", count);
    Ok(())
}

fn validate_password(target: char, password: &str, first: i32, second: i32) -> bool {
    if password.chars().nth(first as usize).unwrap() == target
        && password.chars().nth(second as usize).unwrap() != target
    {
        return true;
    }

    if password.chars().nth(first as usize).unwrap() != target
        && password.chars().nth(second as usize).unwrap() == target
    {
        return true;
    }

    false
}
