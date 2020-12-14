use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn part_1() -> std::io::Result<()> {
    let target_value = 2020;
    let mut visited_expenses = HashSet::new();

    let filename = "files/expense_report.txt";
    let file = File::open(filename)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // println!("{}", contents);

    let reader = BufReader::new(file);

    // for (index, line) in reader.lines().enumerate() {
    //     let line = line.unwrap();
    //     println!("{}. {}", index + 1, line);
    // }

    for line in reader.lines() {
        let expense: i32 = line.unwrap().trim().parse().unwrap();
        let potential_match = target_value - expense;
        if visited_expenses.contains(&potential_match) {
            let product = potential_match * expense;
            println!("{} + {} = {}.", potential_match, expense, target_value);
            println!("{} * {} = {}.", potential_match, expense, product);
            break;
        }

        visited_expenses.insert(expense);
    }

    Ok(())
}
