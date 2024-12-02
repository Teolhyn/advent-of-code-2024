use std::fs::File;
use std::io::{self, BufRead};

fn read_file_to_vectors(path: &str) -> io::Result<Vec<Vec<i32>>> {
    let mut vec1 = Vec::new();

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect();
        vec1.push(numbers);
    }
    Ok(vec1)
}

fn main() {
    let file_path = "input.txt";
    match read_file_to_vectors(file_path) {
        Ok(vec1) => {
            let mut vec_diff: Vec<Vec<i32>> = Vec::new();
            for vec in vec1 {
                let diff = vec.windows(2).map(|s| s[1] - s[0]).collect();
                vec_diff.push(diff);
            }
            let mut safe_count = 0;
            for vec in vec_diff {
                if vec.iter().all(|&x| x < 0 && x > -4) || (vec.iter().all(|&x| x > 0 && x < 4)) {
                    safe_count = safe_count + 1;
                }
            }
            println!("{:?}", safe_count);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
