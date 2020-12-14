use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use lazy_static::*;
use regex::Regex;

use itertools::Itertools;

pub const PARTS: [fn(); 2] = [part1, part2];

lazy_static! {
    static ref MASK_REG: Regex = Regex::new(r"mask = (.+)").unwrap();
    static ref MEM_REG: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

fn part1() {
    let mut mask = vec![None; 36];
    let mut mem = HashMap::new();
    for l in BufReader::new(File::open("inputfiles/day14/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        if let Some(c) = MASK_REG.captures(&l) {
            for (i, x) in c[1].chars().rev().enumerate() {
                mask[i] = match x {
                    'X' => None,
                    '0' => Some(false),
                    '1' => Some(true),
                    _ => unreachable!(),
                }
            }
        } else if let Some(c) = MEM_REG.captures(&l) {
            let i: u64 = c[1].parse().unwrap();
            let mut n: u64 = c[2].parse().unwrap();
            //
            for (i, m) in mask
                .iter()
                .enumerate()
                .filter_map(|(i, x)| x.and_then(|x| Some((i, x))))
            {
                if m {
                    n |= 1 << i;
                } else {
                    n &= !(1 << i);
                }
            }
            //
            if let Some(x) = mem.get_mut(&i) {
                *x = n;
            } else {
                mem.insert(i, n);
            }
        }
    }
    //
    let ans: u64 = mem.values().sum();
    println!("{}", ans);
}

fn part2() {
    let mut mask = vec![None; 36];
    let mut mem = HashMap::new();
    let mut floating = Vec::new();
    for l in BufReader::new(File::open("inputfiles/day14/input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
    {
        if let Some(c) = MASK_REG.captures(&l) {
            for (i, x) in c[1].chars().rev().enumerate() {
                mask[i] = match x {
                    'X' => None,
                    '0' => Some(false),
                    '1' => Some(true),
                    _ => unreachable!(),
                }
            }
        } else if let Some(c) = MEM_REG.captures(&l) {
            let mut address: u64 = c[1].parse().unwrap();
            let n: u64 = c[2].parse().unwrap();
            //
            for (i, x) in mask.iter().enumerate() {
                match x {
                    None => address &= !(1 << i),
                    Some(true) => address |= 1 << i,
                    Some(false) => (),
                };
            }
            //
            floating.clear();
            floating.extend(mask.iter().enumerate().filter_map(|(i, x)| {
                if let None = x {
                    Some(i)
                } else {
                    None
                }
            }));
            //
            for off in (0..=floating.len()).flat_map(|k| {
                floating.iter().combinations(k).map(|comb| {
                    comb.into_iter().map(|&i| 2u64.pow(i as u32)).sum::<u64>()
                })
            }) {
                let n_address = address + off;
                mem.insert(n_address, n);
            }
        }
    }
    //
    let ans: u64 = mem.values().sum();
    println!("{}", ans);
}
