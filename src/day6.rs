use std::{collections::HashSet, fs::read_to_string};

use lazy_static::*;
use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG1: Regex = Regex::new(r"(?:\w\s?)+").unwrap();
}

fn part1() {
    let inp = read_to_string("inputfiles/day6/input.txt").unwrap();
    //
    let mut set = HashSet::new();
    //
    let ans: usize = REG1
        .captures_iter(&inp)
        .map(|c| {
            set.clear();
            set.extend(c[0].chars().filter(|c| !c.is_whitespace()));
            set.len()
        })
        .sum();
    //
    println!("{}", ans);
}

fn part2() {
    let inp = read_to_string("inputfiles/day6/input.txt").unwrap();
    //
    let mut set = HashSet::new();
    let mut agreed1: Vec<char> = Vec::new();
    let mut agreed2: Vec<char> = Vec::new();
    //
    let mut a = &mut agreed1;
    let mut b = &mut agreed2;
    //
    let ans: usize = REG1
        .captures_iter(&inp)
        .map(|c| {
            let mut first = true;
            for l in c[0].lines() {
                set.clear();
                set.extend(l.chars());
                if first {
                    first = false;
                    b.clear();
                    b.extend(set.iter());
                } else {
                    b.clear();
                    b.extend(a.iter().filter(|&c| set.contains(c)));
                }
                std::mem::swap(&mut a, &mut b);
            }
            a.len()
        })
        .sum();
    //
    println!("{}", ans);
}
