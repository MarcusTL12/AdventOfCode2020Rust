use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> Vec<u64> {
    BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|l| l.unwrap().parse().unwrap())
            .collect()
}

fn part1() {
    let numbers = load_input("inputfiles/day1/input.txt");
    //
    let (a, b) = numbers
        .iter()
        .cartesian_product(numbers.iter())
        .filter(|&(a, b)| a + b == 2020)
        .next()
        .unwrap();
    //
    println!("{}", a * b);
}

fn part2() {
    let numbers = load_input("inputfiles/day1/input.txt");
    //
    let ((a, b), c) = numbers
        .iter()
        .cartesian_product(numbers.iter())
        .cartesian_product(numbers.iter())
        .filter(|&((a, b), c)| a + b + c == 2020)
        .next()
        .unwrap();
    //
    println!("{}", a * b * c);
}
