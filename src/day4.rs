use crate::reader::read_txt;

pub fn part_1() -> std::io::Result<()> {
    let filename = "files/day4.txt";
    let data = read_txt(filename)?;
    // println!("{:?}", data);
    for line in &data {
        println!("{}", *line);
    }

    Ok(())
}
