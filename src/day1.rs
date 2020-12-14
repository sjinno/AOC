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
            // println!("{} + {} = {}.", potential_match, expense, target_value);
            // println!("{} * {} = {}.", potential_match, expense, product);
            println!(
                "Answer for part 1: {} * {} = {}.",
                potential_match, expense, product
            );
            break;
        }

        visited_expenses.insert(expense);
    }

    Ok(())
}

pub fn part_2() -> std::io::Result<()> {
    let target_value = 2020;
    let filename = "files/expense_report.txt";
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut expense_report: Vec<i32> = vec![];
    for line in reader.lines() {
        let expense: String = line.unwrap().trim().parse().unwrap();
        if expense.is_empty() {
            break;
        }
        expense_report.push(expense.parse::<i32>().unwrap());
    }

    let three_numbers = three_number_sum(target_value, expense_report);
    let res = three_numbers.iter().fold(1, |acc, x| acc * x);
    if res == -1 {
        println!("There are no such pairs. :(");
    } else {
        println!(
            "Answer for part 2: {} * {} * {} = {}.",
            three_numbers[0], three_numbers[1], three_numbers[2], res
        );
    }

    Ok(())
}

fn three_number_sum(target: i32, v: Vec<i32>) -> [i32; 3] {
    let mut visited = HashSet::new();
    for i in 0..v.len() {
        let temp_target = target - v[i];
        for j in (i + 1)..v.len() {
            let potential_match = temp_target - v[j];
            if visited.contains(&potential_match) {
                // println!("{} + {} + {} = {}", v[i], v[j], potential_match, target);
                // println!("{} * {} * {} = {}", v[i], v[j], potential_match, product);
                return [v[i], v[j], potential_match];
            }
            visited.insert(v[j]);
        }
    }
    [-1, -1, -1]
}
