use std::{collections::HashMap, fs};

use lazy_static::*;
use regex::Regex;

lazy_static! {
    static ref NLINE: Regex = Regex::new(r"(?:\r?\n){2}").unwrap();
    static ref RULE_IND_REG: Regex = Regex::new(r"(\d+): (.+)").unwrap();
    static ref RULE_CHAR_REG: Regex = Regex::new("\"(.)\"").unwrap();
    static ref RULE_OR_REG: Regex = Regex::new(r"(.+) \| (.+)").unwrap();
    static ref RULE_ARRAY_REG: Regex = Regex::new(r"((?:\d+ ?)+)").unwrap();
}

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug)]
enum Rule {
    Char(u8),
    Array(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

fn parse_array(s: &str) -> Vec<usize> {
    s.split(' ').map(|s| s.parse().unwrap()).collect()
}

fn load_input(filename: &str) -> (HashMap<usize, Rule>, Vec<Vec<u8>>) {
    let s = fs::read_to_string(filename).unwrap();
    //
    let mut parts = NLINE.split(&s);
    //
    let rules = parts
        .next()
        .unwrap()
        .lines()
        .filter_map(|l| RULE_IND_REG.captures(&l))
        .map(|c| {
            let i = c[1].parse().unwrap();
            //
            (
                i,
                if let Some(c) = RULE_CHAR_REG.captures(&c[2]) {
                    Rule::Char(c[1].chars().next().unwrap() as u8)
                } else if let Some(c) = RULE_OR_REG.captures(&c[2]) {
                    Rule::Or(parse_array(&c[1]), parse_array(&c[2]))
                } else if let Some(c) = RULE_ARRAY_REG.captures(&c[2]) {
                    Rule::Array(parse_array(&c[1]))
                } else {
                    unreachable!()
                },
            )
        })
        .collect();
    //
    let messages = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| Vec::from(l.as_bytes()))
        .collect();
    //
    (rules, messages)
}

fn matches_rule<'a>(
    rules: &HashMap<usize, Rule>,
    rule: &Rule,
    s: &'a [u8],
) -> Option<&'a [u8]> {
    //
    fn matches_array<'a>(
        rules: &HashMap<usize, Rule>,
        v: &Vec<usize>,
        s: &'a [u8],
    ) -> Option<&'a [u8]> {
        v.iter().fold(Some(s), |s, ind| {
            s.and_then(|s| matches_rule(rules, &rules[ind], s))
        })
    }
    //
    match rule {
        Rule::Char(c) => {
            if s.get(0) == Some(c) {
                Some(&s[1..])
            } else {
                None
            }
        }
        Rule::Array(v) => matches_array(rules, v, s),
        Rule::Or(v1, v2) => match matches_array(rules, v1, s) {
            Some(s) => Some(s),
            None => matches_array(rules, v2, s),
        },
    }
}

fn is_match(rules: &HashMap<usize, Rule>, rule: &Rule, s: &[u8]) -> bool {
    matches!(matches_rule(rules, rule, s), Some(&[]))
}

#[derive(Debug)]
enum MatchResult<'a> {
    Single(&'a [u8]),
    Multiple(Vec<&'a [u8]>),
}

fn matches_rule2<'a>(
    rules: &HashMap<usize, Rule>,
    rule: &Rule,
    s: &'a [u8],
) -> Option<MatchResult<'a>> {
    //
    fn matches_array<'a>(
        rules: &HashMap<usize, Rule>,
        v: &Vec<usize>,
        s: &'a [u8],
    ) -> Option<MatchResult<'a>> {
        let mut res = Some(MatchResult::Single(s));
        //
        for rule in v.iter().map(|i| &rules[i]) {
            res = match res {
                Some(MatchResult::Single(s)) => matches_rule2(rules, rule, s),
                Some(MatchResult::Multiple(v)) => {
                    let mut nv = Vec::new();
                    for x in v.iter().map(|&s| matches_rule2(rules, rule, s)) {
                        match x {
                            Some(MatchResult::Single(s)) => nv.push(s),
                            Some(MatchResult::Multiple(ov)) => nv.extend(ov),
                            None => (),
                        }
                    }
                    match nv.len() {
                        0 => None,
                        1 => Some(MatchResult::Single(nv[0])),
                        _ => Some(MatchResult::Multiple(nv)),
                    }
                }
                None => break,
            };
        }
        //
        res
    }
    //
    match rule {
        Rule::Char(c) => {
            if s.get(0) == Some(c) {
                Some(MatchResult::Single(&s[1..]))
            } else {
                None
            }
        }
        Rule::Array(v) => matches_array(rules, v, s),
        Rule::Or(v1, v2) => {
            let mut nv = Vec::new();
            //
            for v in &[v1, v2] {
                match matches_array(rules, v, s) {
                    Some(MatchResult::Single(s)) => nv.push(s),
                    Some(MatchResult::Multiple(ov)) => nv.extend(ov),
                    None => (),
                }
            }
            //
            match nv.len() {
                0 => None,
                1 => Some(MatchResult::Single(nv[0])),
                _ => Some(MatchResult::Multiple(nv)),
            }
        }
    }
}

fn is_match2(rules: &HashMap<usize, Rule>, rule: &Rule, s: &[u8]) -> bool {
    match matches_rule2(rules, rule, s) {
        Some(MatchResult::Single(&[])) => true,
        Some(MatchResult::Multiple(v)) => v.iter().any(|s| s.len() == 0),
        _ => false
    }
}

fn part1() {
    let (rules, messages) = load_input("inputfiles/day19/input.txt");
    //
    let rule_zero = &rules[&0];
    //
    let ans = messages
        .into_iter()
        .filter(|s| is_match(&rules, rule_zero, s))
        .count();
    //
    println!("{}", ans);
}

fn part2() {
    let (mut rules, messages) = load_input("inputfiles/day19/input.txt");
    //
    if let Some(x) = rules.get_mut(&8) {
        *x = Rule::Or(vec![42], vec![42, 8]);
    }
    if let Some(x) = rules.get_mut(&11) {
        *x = Rule::Or(vec![42, 31], vec![42, 11, 31]);
    }
    //
    let rule_zero = &rules[&0];
    //
    let ans = messages
        .into_iter()
        .filter(|s| is_match2(&rules, rule_zero, s))
        .count();
    //
    println!("{}", ans);
}
