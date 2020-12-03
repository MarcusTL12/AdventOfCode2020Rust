use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use lazy_static::*;
use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
}

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day2/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| {
            if let Some(c) = REG.captures(&l) {
                let a = c[1].parse().unwrap();
                let b = c[2].parse().unwrap();
                let ch = c[3].chars().next().unwrap();
                let x = c[4].chars().filter(|&x| x == ch).count();
                //
                x >= a && x <= b
            } else {
                unreachable!()
            }
        })
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let ans = BufReader::new(File::open("inputfiles/day2/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| {
            if let Some(c) = REG.captures(&l) {
                let a = c[1].parse().unwrap();
                let b = c[2].parse().unwrap();
                let ch = c[3].chars().next().unwrap() as u8;
                //
                [a, b]
                    .iter()
                    .filter(|&&x: &&usize| c[4].as_bytes()[x - 1] == ch)
                    .count()
                    == 1
            } else {
                unreachable!()
            }
        })
        .count();
    //
    println!("{}", ans);
}
