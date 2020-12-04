use std::{collections::HashMap, fs::read_to_string};

use lazy_static::*;
use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref REG1: Regex = Regex::new(r"(?:\w{3}:.+\s?)+").unwrap();
    static ref REG2: Regex = Regex::new(r"(\w{3}):(\S+)").unwrap();
    static ref REG_HEIGHT: Regex = Regex::new(r"(\d+)(\w{2})").unwrap();
    static ref REG_COLOR: Regex = Regex::new(r"[a-f0-9]{6}").unwrap();
    static ref REG_PID: Regex = Regex::new(r"\d{9}").unwrap();
}

fn part1() {
    let s = read_to_string("inputfiles/day4/input.txt").unwrap();
    //
    let mut map: HashMap<_, _> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .map(|&s| (s, false))
            .collect();
    //
    let ans = REG1
        .captures_iter(&s)
        .filter(|c| {
            map.values_mut().for_each(|x| *x = false);
            for c in REG2.captures_iter(&c[0]) {
                if let Some(x) = map.get_mut(&c[1]) {
                    *x = true;
                }
            }
            map.values().all(|&x| x)
        })
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let s = read_to_string("inputfiles/day4/input.txt").unwrap();
    //
    let ans = REG1
        .captures_iter(&s)
        .filter(|c| {
            let mut fields = [false; 7];
            for c in REG2.captures_iter(&c[0]) {
                match &c[1] {
                    "byr" => {
                        let i: u32 = c[2].parse().unwrap();
                        fields[0] = i >= 1920 && i <= 2002;
                    }
                    "iyr" => {
                        let i: u32 = c[2].parse().unwrap();
                        fields[1] = i >= 2010 && i <= 2020;
                    }
                    "eyr" => {
                        let i: u32 = c[2].parse().unwrap();
                        fields[2] = i >= 2020 && i <= 2030;
                    }
                    "hgt" => {
                        if let Some(c) = REG_HEIGHT.captures(&c[2]) {
                            match &c[2] {
                                "cm" => {
                                    let i: u32 = c[1].parse().unwrap();
                                    fields[3] = i >= 150 && i <= 193;
                                }
                                "in" => {
                                    let i: u32 = c[1].parse().unwrap();
                                    fields[3] = i >= 59 && i <= 76;
                                }
                                _ => (),
                            }
                        }
                    }
                    "hcl" => {
                        fields[4] = c[2].len() == 7 && REG_COLOR.is_match(&c[2])
                    }
                    "ecl" => {
                        fields[5] = matches!(
                            &c[2],
                            "amb"
                                | "blu"
                                | "brn"
                                | "gry"
                                | "grn"
                                | "hzl"
                                | "oth"
                        )
                    }
                    "pid" => {
                        fields[6] = c[2].len() == 9 && REG_PID.is_match(&c[2])
                    }
                    "cid" => (),
                    _ => unreachable!(),
                }
            }
            //
            fields.iter().all(|&x| x)
        })
        .count();
    //
    println!("{}", ans);
}
