use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use lazy_static::*;
use regex::Regex;

use num_complex::Complex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"(\w)(\d+)").unwrap();
}

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut pos = Complex { re: 0, im: 0 };
    let mut dir = Complex { re: 1, im: 0 };
    for (c, n) in
        BufReader::new(File::open("inputfiles/day12/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let Some(c) = REG.captures(&l) {
                    let ch = c[1].chars().next().unwrap();
                    let n: i64 = c[2].parse().unwrap();
                    //
                    (ch, n)
                } else {
                    unreachable!()
                }
            })
    {
        match c {
            'N' => pos += n * Complex { re: 0, im: 1 },
            'S' => pos += n * Complex { re: 0, im: -1 },
            'E' => pos += n * Complex { re: 1, im: 0 },
            'W' => pos += n * Complex { re: -1, im: 0 },
            'F' => pos += n * dir,
            'L' | 'R' => {
                let sign = match c {
                    'L' => 1,
                    'R' => -1,
                    _ => unreachable!(),
                };
                dir *= match n {
                    90 => Complex { re: 0, im: sign },
                    180 => Complex { re: -1, im: 0 },
                    270 => Complex { re: 0, im: -sign },
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    //
    let ans = pos.re.abs() + pos.im.abs();
    //
    println!("{}", ans);
}

fn part2() {
    let mut pos = Complex { re: 0, im: 0 };
    let mut waypoint = Complex { re: 10, im: 1 };
    for (c, n) in
        BufReader::new(File::open("inputfiles/day12/input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let Some(c) = REG.captures(&l) {
                    let ch = c[1].chars().next().unwrap();
                    let n: i64 = c[2].parse().unwrap();
                    //
                    (ch, n)
                } else {
                    unreachable!()
                }
            })
    {
        match c {
            'N' => waypoint += n * Complex { re: 0, im: 1 },
            'S' => waypoint += n * Complex { re: 0, im: -1 },
            'E' => waypoint += n * Complex { re: 1, im: 0 },
            'W' => waypoint += n * Complex { re: -1, im: 0 },
            'F' => pos += n * waypoint,
            'L' | 'R' => {
                let sign = match c {
                    'L' => 1,
                    'R' => -1,
                    _ => unreachable!(),
                };
                waypoint *= match n {
                    90 => Complex { re: 0, im: sign },
                    180 => Complex { re: -1, im: 0 },
                    270 => Complex { re: 0, im: -sign },
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    //
    let ans = pos.re.abs() + pos.im.abs();
    //
    println!("{}", ans);
}
