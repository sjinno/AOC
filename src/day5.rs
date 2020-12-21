use crate::reader::{clean_input, read_txt};
use std::collections::HashSet;

#[allow(unused_macros)]
// Debug format.
macro_rules! printd {
    ($v:expr) => {
        println!("{:?}", $v);
    };
}

#[allow(unused_macros)]
// Format.
macro_rules! printf {
    ($v:expr) => {
        println!("{}", $v);
    };
}

pub fn part1() -> std::io::Result<()> {
    let filename = "files/day5.txt";
    let lines = read_txt(filename)?;
    // let cleaned_input = clean_input(lines);
    // printf!(cleaned_input);
    let mut highest_id = 0;
    // printd!(lines);
    for seat in lines {
        // printf!(seat);
        let id = get_id(&seat);
        if id > highest_id {
            highest_id = id;
        }
    }
    printf!(highest_id);
    Ok(())
}

fn get_id(seat: &str) -> u32 {
    const NUMBER_OF_ROWS: u32 = 128;
    const NUMBER_OF_COLS: u32 = 8;

    let row_dividers: Vec<u32> = (0..NUMBER_OF_COLS - 1)
        .rev()
        .map(|d| 2_u32.pow(d))
        .collect();
    let col_dividers: Vec<u32> = (0..3).rev().map(|d| 2_u32.pow(d)).collect();
    // println!("{:?}", col_dividers);

    let mut f = 0;
    let mut b = NUMBER_OF_ROWS - 1;
    let mut l = 0;
    let mut r = NUMBER_OF_COLS - 1;
    let mut i = 0; // Row counter.
    let mut j = 0; // Col counter.
    let mut row = 0;
    let mut col = 0;
    for c in seat.chars() {
        if i == 6 {
            if c == 'F' {
                row = f;
            } else if c == 'B' {
                row = b;
            }
        }

        if j == 2 {
            if c == 'L' {
                col = l;
            } else if c == 'R' {
                col = r;
            }
        }
        match c {
            'F' => {
                b -= row_dividers[i];
                i += 1;
            }
            'B' => {
                f += row_dividers[i];
                i += 1;
            }
            'L' => {
                r -= col_dividers[j];
                j += 1;
            }
            'R' => {
                l += col_dividers[j];
                j += 1;
            }
            _ => {}
        }
    }
    // println!("{}", row);
    // println!("{}", col);

    row * 8 + col
}

pub fn part2() -> std::io::Result<()> {
    let filename = "files/day5.txt";
    let lines = read_txt(filename)?;
    let mut my_list: HashSet<u32> = HashSet::new();
    let mut seats_taken: HashSet<u32> = HashSet::new();
    let mut highest = 0;
    let mut lowest = 0;
    let mut get_lowest = true;
    for seat in &lines {
        let id = get_id(&seat);
        if get_lowest {
            get_lowest = false;
            lowest = id;
        }
        if highest < id {
            highest = id;
        } else if lowest > id {
            lowest = id;
        }
        seats_taken.insert(id);
        my_list.insert(id + 1);
        my_list.insert(id - 1);
    }

    // printf!(highest);
    // printf!(lowest);

    for x in my_list.iter() {
        if x < &lowest || x > &highest {
            continue;
        }
        if !seats_taken.contains(x) {
            printf!(x);
        }
    }
    Ok(())
}
