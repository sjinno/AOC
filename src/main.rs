mod day1;

fn main() {
    if let Err(e) = day1::part_1() {
        println!("Error occured: {}.", e.to_string());
    }
    if let Err(e) = day1::part_2() {
        println!("Error occured: {}.", e.to_string());
    }
}
