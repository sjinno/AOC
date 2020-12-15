// mod day1;
mod day2;

fn main() {
    // // DAY 1
    // if let Err(e) = day1::part_1() {
    //     println!("Error occured: {}.", e.to_string());
    // }
    // if let Err(e) = day1::part_2() {
    //     println!("Error occured: {}.", e.to_string());
    // }

    // DAY 2
    if let Err(e) = day2::password_philosophy() {
        println!("Error occured: {}.", e.to_string());
    }
    if let Err(e) = day2::part_2() {
        println!("Error occured: {}.", e.to_string());
    }
}
