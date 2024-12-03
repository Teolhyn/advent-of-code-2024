use regex::Regex;
use std::fs::read_to_string;
use std::io;

fn main() -> Result<(), io::Error> {
    let input = read_to_string("input.txt")?;

    let regex = Regex::new(r"\bdo(?:n't)?\b|mul\((-?\d+),(-?\d+)\)").unwrap(); // -? -> allows negatives, \d+ -> one or more digits.
                                                                               // (-?\d+) -> any digit.
    let mut result = 0;
    let mut do_flag = true;

    for hit in regex.captures_iter(&input) {
        if let Some(do_match) = hit.get(0) {
            let matched_text = do_match.as_str();

            if matched_text == "do" {
                do_flag = true;
            } else if matched_text == "don't" {
                do_flag = false;
            } else if hit.get(1).is_some() && hit.get(2).is_some() {
                let number1: i32 = hit[1].parse().unwrap();
                let number2: i32 = hit[2].parse().unwrap();

                if do_flag {
                    result = result + number1 * number2;
                }
            }
        }
    }

    println!("{:?}", result);
    Ok(())
}
