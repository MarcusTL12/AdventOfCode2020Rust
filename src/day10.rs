use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> Vec<u64> {
    let mut inp: Vec<u64> = BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    //
    inp.push(*inp.iter().max().unwrap() + 3);
    inp.push(0);
    inp.sort_unstable();
    inp
}

fn part1() {
    let inp = load_input("inputfiles/day10/input.txt");
    //
    let (a, b) = inp.iter().tuple_windows().map(|(a, b)| b - a).fold(
        (0, 0),
        |(a, b), d| {
            if d == 1 {
                (a + 1, b)
            } else if d == 3 {
                (a, b + 1)
            } else {
                (a, b)
            }
        },
    );
    //
    println!("{}", a * b);
    // println!("1: {}, 3: {}", a, b);
}

fn part2() {
    let inp = load_input("inputfiles/day10/input.txt");
    //
    let mut amt_combs = vec![0u64; inp.len()];
    amt_combs[inp.len() - 1] = 1;
    //
    for (i, jolts) in inp.iter().enumerate().rev().skip(1) {
        amt_combs[i] = (1..=3)
            .filter_map(|j| {
                inp.get(i + j).and_then(|x| {
                    let d = x - jolts;
                    if 1 <= d && d <= 3 {
                        amt_combs.get(i + j)
                    } else {
                        None
                    }
                })
            })
            .sum();
    }
    //
    println!("{}", amt_combs[0]);
}
