use regex::Regex;

use crate::reader::read_txt;

const FIELDS: [&'static str; 8] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

// pub fn part_1() -> std::io::Result<()> {
//     let filename = "files/day4.txt";
//     let lines = read_txt(filename)?;
//     let contents = lines.join("\n");
//     let passports = contents.split("\n\n").collect::<Vec<&str>>();

//     // let mut passes = vec![];

//     let mut count = 0;
//     let number_of_fields = FIELDS.len() - 1;
//     for passport in &passports {
//         let mut f_count = 0;
//         for f in &FIELDS {
//             if *f == "cid" {
//                 continue;
//             }
//             if passport.contains(*f) {
//                 // println!("true");
//                 f_count += 1;
//             }
//             if f_count == number_of_fields {
//                 count += 1;
//             }
//         }
//     }
//     println!("Number of valid passports: {}.", count);
//     Ok(())
// }

#[allow(unused_macros)]
macro_rules! printd {
    ($v:expr) => {
        println!("{:?}", $v);
    };
}

#[allow(unused_macros)]
macro_rules! printf {
    ($v:expr) => {
        println!("{}", $v);
    };
}

macro_rules! validate {
    ($b:expr, $c:expr) => {
        if $b {
            $c += 1;
        } else {
            break;
        }
    };
}

fn validate_byr(byr: &str) -> bool {
    let re = Regex::new(r"([1][9][2-9][0-9])|([2][0][0][0-2])").unwrap();
    re.is_match(byr)
}

fn validate_iyr(iyr: &str) -> bool {
    let re = Regex::new(r"[2][0]([1][0-9]|[2][0])").unwrap();
    re.is_match(iyr)
}

fn validate_eyr(eyr: &str) -> bool {
    let re = Regex::new(r"[2][0]([2][0-9]|[3][0])").unwrap();
    re.is_match(eyr)
}

fn validate_hgt(hgt: &str) -> bool {
    let re = Regex::new(r"(\b[1]([5-8][0-9]|[9][0-3])(cm)\b)|(\b([5][9]|[6][0-9]|[7][0-6])(in)\b)")
        .unwrap();
    re.is_match(hgt)
}

fn validate_hcl(hcl: &str) -> bool {
    let re = Regex::new(r"\B(#)(([0-9]|[a-f]){6})\b").unwrap();
    re.is_match(hcl)
}

fn validate_ecl(ecl: &str) -> bool {
    let ecls: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    ecls.iter().any(|i| *i == ecl)
}

fn validate_pid(pid: &str) -> bool {
    let re = Regex::new(r"\b[0-9]{9}\b").unwrap();
    re.is_match(pid)
}

pub fn part_2() -> std::io::Result<()> {
    let filename = "files/day4.txt";
    let lines = read_txt(filename)?;
    let contents: &str = &lines.join("\n");
    let cleaned_data = str::replace(contents, " ", "\n");
    let number_of_fields = FIELDS.len() - 1;
    let passports = cleaned_data.split("\n\n").collect::<Vec<&str>>();
    let mut count = 0;
    for passport in &passports {
        let mut f_count = 0;
        let fields = passport.split("\n").collect::<Vec<&str>>();
        for field in fields {
            let entry = field.split(":").collect::<Vec<&str>>();
            match entry[0] {
                "cid" => continue,
                "byr" => validate!(validate_byr(entry[1]), f_count),
                "iyr" => validate!(validate_iyr(entry[1]), f_count),
                "eyr" => validate!(validate_eyr(entry[1]), f_count),
                "hgt" => validate!(validate_hgt(entry[1]), f_count),
                "hcl" => validate!(validate_hcl(entry[1]), f_count),
                "ecl" => validate!(validate_ecl(entry[1]), f_count),
                "pid" => validate!(validate_pid(entry[1]), f_count),
                _ => {}
            }
        }
        if f_count == number_of_fields {
            count += 1;
        }
    }
    printf!(count);
    Ok(())
}
