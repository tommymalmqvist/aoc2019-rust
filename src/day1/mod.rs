use std::fs;

pub fn day1() -> i32 {
    let filename = "src/day1/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let numbers: Vec<i32> = contents
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut res = 0;
    for num in numbers {
        res += (num / 3) - 2;
    }

    return res;
}
