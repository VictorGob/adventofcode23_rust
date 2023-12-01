use regex::Regex;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
fn main() {
    let path = "input.txt";
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"(\d+)").unwrap();
    let mut total: i32 = 0;
    for line in reader.lines() {
        let mut _num = String::new();
        for cap in re.captures_iter(&line.unwrap()) {
            _num += &cap[0];
        }
        match _num.len().cmp(&2) {
            Ordering::Greater => {
                let first = _num.chars().next().unwrap().to_digit(10).unwrap() as i32;
                let last = _num
                    .chars()
                    .nth(_num.len() - 1)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32;
                total += first * 10 + last;
            }
            Ordering::Less => {
                total += format!("{}{}", _num, _num).parse::<i32>().unwrap();
            }
            Ordering::Equal => {
                total += _num.parse::<i32>().unwrap();
            }
        }
    }

    println!("Total: {}", total)
}
