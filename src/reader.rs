use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_txt(filename: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut data = vec![];
    for line in reader.lines() {
        // println!("{}", line.unwrap());
        data.push(line.unwrap());
    }
    // let data = reader.lines().collect::<String>();
    Ok(data)
}
