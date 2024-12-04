use std::fs::read_to_string;

fn string_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_x_patterns(matrix: &Vec<Vec<char>>) -> i32 {
    let n = matrix.len();
    let mut x_count = 0;

    for i in 1..(n - 1) {
        for j in 1..(n - 1) {
            let center = matrix[i][j];
            let top_left = matrix[i - 1][j - 1];
            let top_right = matrix[i - 1][j + 1];
            let bottom_left = matrix[i + 1][j - 1];
            let bottom_right = matrix[i + 1][j + 1];

            println!(
                "Checking cell ({}, {}): center = '{}', top_left = '{}', top_right = '{}', bottom_left = '{}', bottom_right = '{}'",
                i, j, center, top_left, top_right, bottom_left, bottom_right
            );

            let pattern_1 = top_left == 'M'
                && top_right == 'S'
                && center == 'A'
                && bottom_left == 'M'
                && bottom_right == 'S';
            let pattern_2 = top_left == 'S'
                && top_right == 'M'
                && center == 'A'
                && bottom_left == 'S'
                && bottom_right == 'M';
            let pattern_3 = top_left == 'S'
                && top_right == 'S'
                && center == 'A'
                && bottom_left == 'M'
                && bottom_right == 'M';
            let pattern_4 = top_left == 'M'
                && top_right == 'M'
                && center == 'A'
                && bottom_left == 'S'
                && bottom_right == 'S';

            if pattern_1 || pattern_2 || pattern_3 || pattern_4 {
                x_count += 1;
            }
        }
    }

    x_count
}

fn main() -> Result<(), std::io::Error> {
    let input = read_to_string("input.txt");
    let matrix = string_to_matrix(&input?);
    println!("{:?}", matrix);

    println!("{:?}", find_x_patterns(&matrix));

    Ok(())
}
