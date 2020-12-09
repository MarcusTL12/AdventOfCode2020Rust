use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

const PREAMBLE: usize = 25;

fn first_invalid(filename: &str) -> Option<usize> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .scan(VecDeque::with_capacity(PREAMBLE), |nums, n| {
            if nums.len() < PREAMBLE {
                nums.push_back(n);
                Some(None)
            } else {
                let tmp = if nums
                    .iter()
                    .tuple_combinations()
                    .filter_map(
                        |(a, b)| {
                            if a + b == n {
                                Some(())
                            } else {
                                None
                            }
                        },
                    )
                    .next()
                    .is_some()
                {
                    Some(None)
                } else {
                    Some(Some(n))
                };
                nums.pop_front();
                nums.push_back(n);
                tmp
            }
        })
        .filter_map(|x| x)
        .next()
}

fn part1() {
    let ans = first_invalid("inputfiles/day9/input.txt");
    //
    println!("{:?}", ans);
}

fn part2() {
    let file = "inputfiles/day9/input.txt";
    let invalid = first_invalid(file).unwrap();
    //
    let nums: Vec<usize> = BufReader::new(File::open(file).unwrap())
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    //
    for i in 1..nums.len() {
        for j in i + 1..nums.len() {
            let s: usize = nums[i..j].iter().sum();
            if s > invalid {
                break;
            } else if s == invalid {
                let ans = nums[i..j].iter().min().and_then(|x| {
                    nums[i..j].iter().max().and_then(|y| Some(x + y))
                });
                println!("{:?}", ans);
                return;
            }
        }
    }
}
