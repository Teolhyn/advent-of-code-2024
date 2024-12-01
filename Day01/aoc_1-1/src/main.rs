use std::fs::File;
use std::io::{self, BufRead};

fn read_file_to_vectors(path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split("   ").collect();
        if numbers.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                vec1.push(num1);
                vec2.push(num2);
            }
        }
    }

    Ok((vec1, vec2))
}

fn main() {
    let file_path = "input.txt";
    match read_file_to_vectors(file_path) {
        Ok((mut vec1, mut vec2)) => {
            vec1.sort();
            vec2.sort();
            let vec3: Vec<i32> = vec1
                .iter()
                .zip(vec2.iter())
                .map(|(a, b)| (a - b).abs())
                .collect();
            let result: i32 = vec3.iter().sum();
            println!("{:?}", result);
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
