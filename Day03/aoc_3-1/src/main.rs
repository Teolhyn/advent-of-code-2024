use regex::Regex;
use std::fs::read_to_string;
use std::io;

fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?;

    let regex = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap(); // -? -> allows negatives, \d+ -> one or more digits.
                                                                // (-?\d+) -> any digit.

    let mut result = 0;

    for hit in regex.captures_iter(&input) {
        let number1: i32 = hit[1].parse().unwrap();
        let number2: i32 = hit[2].parse().unwrap();

        result = result + number1 * number2;
    }
    println!("{:?}", result);
    Ok(())
}
