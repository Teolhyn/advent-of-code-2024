use std::fs::read_to_string;
use std::usize;

use diagonal::{diagonal_pos_neg, diagonal_pos_pos};

fn string_to_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn count_xmas(matrix: &Vec<Vec<char>>) -> usize {
    let mut n_xmas = 0;

    for row in matrix {
        let xmas: Vec<char> = "XMAS".chars().collect();
        let samx: Vec<char> = "SAMX".chars().collect();
        n_xmas += row.windows(4).filter(|w| w == &xmas || w == &samx).count();
    }

    n_xmas
}

fn main() -> Result<(), std::io::Error> {
    let mut n_xmas = 0;

    let input = read_to_string("input.txt");
    let matrix = string_to_matrix(&input?);

    n_xmas = n_xmas + count_xmas(&matrix);
    println!("In rows {:?}", n_xmas);

    let matrix_t = transpose(matrix.clone());

    n_xmas = n_xmas + count_xmas(&matrix_t);
    println!("In columns {:?}", count_xmas(&matrix_t));

    let matrix_diagonal_pos_pos = diagonal_pos_pos(&matrix);
    println!("{:?}", matrix_diagonal_pos_pos);
    for row in matrix_diagonal_pos_pos {
        let xmas = vec![&'X', &'M', &'A', &'S']; // Yikes
        let samx = vec![&'S', &'A', &'M', &'X'];
        n_xmas += row.windows(4).filter(|w| w == &xmas || w == &samx).count();
    }

    let matrix_diagonal_pos_neg = diagonal_pos_neg(&matrix);

    for row in matrix_diagonal_pos_neg {
        let xmas = vec![&'X', &'M', &'A', &'S']; // Yikes
        let samx = vec![&'S', &'A', &'M', &'X'];
        n_xmas += row.windows(4).filter(|w| w == &xmas || w == &samx).count();
    }

    println!("{:?}", n_xmas);

    Ok(())
}
