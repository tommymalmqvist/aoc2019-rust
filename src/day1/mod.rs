use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn day1() -> i32 {
    let mut res: i32 = 0;

    let file = File::open("src/day1/input.txt").unwrap();

    let buffer  = BufReader::new(file);

    for line in buffer.lines() {
        let num: i32 = line.unwrap().parse().unwrap();
        res += (num / 3) - 2;
    }

    return res

}