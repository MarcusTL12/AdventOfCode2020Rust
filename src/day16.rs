use std::fs;

use lazy_static::*;
use regex::Regex;

lazy_static! {
    static ref DLINE_REG: Regex = Regex::new(r"(?:\r?\n){2}").unwrap();
    static ref RULE_REG: Regex =
        Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(
    filename: &str,
) -> (
    Vec<(String, ((u64, u64), (u64, u64)))>,
    Vec<u64>,
    Vec<Vec<u64>>,
) {
    let s = fs::read_to_string(filename).unwrap();
    //
    let (rules, your, tickets) = {
        let mut it = DLINE_REG.split(&s);
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    };
    //
    let rules = rules
        .lines()
        .map(|l| {
            if let Some(c) = RULE_REG.captures(&l) {
                (
                    c[1].to_owned(),
                    (
                        (c[2].parse().unwrap(), c[3].parse().unwrap()),
                        (c[4].parse().unwrap(), c[5].parse().unwrap()),
                    ),
                )
            } else {
                unreachable!()
            }
        })
        .collect();
    //
    let your = your
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    //
    let tickets = tickets
        .lines()
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    //
    (rules, your, tickets)
}

fn in_range(n: u64, ((a, b), (c, d)): ((u64, u64), (u64, u64))) -> bool {
    n >= a && n <= b || n >= c && n <= d
}

fn part1() {
    let (rules, _, tickets) = load_input("inputfiles/day16/input.txt");
    //
    let ans: u64 = tickets
        .into_iter()
        .flatten()
        .filter(|&x| rules.iter().all(|&(_, r)| !in_range(x, r)))
        .sum();
    //
    println!("{}", ans);
}

fn part2() {
    let (rules, your, mut tickets) = load_input("inputfiles/day16/input.txt");
    //
    tickets.retain(|ticket| {
        ticket
            .iter()
            .all(|&n| rules.iter().any(|&(_, r)| in_range(n, r)))
    });
    //
    let mut sudoku: Vec<Vec<_>> = (0..your.len())
        .map(|i| {
            rules
                .iter()
                .map(|&(_, r)| {
                    tickets
                        .iter()
                        .map(|ticket| ticket[i])
                        .all(|val| in_range(val, r))
                })
                .collect()
        })
        .collect();
    //
    let mut accounted_for = vec![false; your.len()];
    //
    while !accounted_for.iter().all(|&x| x) {
        for i in 0..sudoku.len() {
            if !accounted_for[i]
                && sudoku[i].iter().filter(|&&x| x).count() == 1
            {
                if let Some(k) = sudoku[i]
                    .iter()
                    .enumerate()
                    .filter(|&(_, x)| *x)
                    .map(|(k, _)| k)
                    .next()
                {
                    accounted_for[i] = true;
                    for j in (0..sudoku.len()).filter(|&x| x != i) {
                        sudoku[j][k] = false;
                    }
                }
            }
        }
    }
    //
    let actual_rules: Vec<_> = sudoku
        .into_iter()
        .filter_map(|x| {
            x.into_iter()
                .enumerate()
                .filter(|&(_, x)| x)
                .map(|(i, _)| i)
                .next()
        })
        .collect();
    //
    let ans: u64 = actual_rules
        .into_iter()
        .map(|i| &rules[i])
        .zip(your)
        .filter(|((name, _), _)| name.starts_with("departure"))
        .map(|(_, val)| val)
        .product();
    //
    println!("{}", ans);
}
