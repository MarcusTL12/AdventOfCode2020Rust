use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

use lazy_static::*;
use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"(.+) \(contains (.+)\)").unwrap();
}

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let mut sudoku: HashMap<_, HashSet<_>> = HashMap::new();
    let mut appearences = HashMap::new();
    let mut ings = HashSet::new();
    let mut to_delete = Vec::new();
    //
    let s = fs::read_to_string("inputfiles/day21/input.txt").unwrap();
    for l in s.lines() {
        if let Some(c) = REG.captures(&l) {
            ings.clear();
            ings.extend(c.get(1).unwrap().as_str().split(' '));
            //
            for &ing in &ings {
                if let Some(x) = appearences.get_mut(ing) {
                    *x += 1
                } else {
                    appearences.insert(ing, 1);
                }
            }
            //
            for allergen in c.get(2).unwrap().as_str().split(", ") {
                if let Some(x) = sudoku.get_mut(allergen) {
                    to_delete.clear();
                    for &x in x.iter() {
                        if !ings.contains(x) {
                            to_delete.push(x);
                        }
                    }
                    for y in &to_delete {
                        x.remove(y);
                    }
                } else {
                    sudoku.insert(allergen, ings.clone());
                }
            }
        }
    }
    //
    let mut without: HashSet<_> = appearences.keys().cloned().collect();
    //
    for &ing in sudoku.values().flatten() {
        without.remove(ing);
    }
    //
    let ans: usize = without.into_iter().map(|ing| appearences[ing]).sum();
    //
    println!("{}", ans);
}

fn part2() {
    let mut sudoku: HashMap<_, HashSet<_>> = HashMap::new();
    let mut ings = HashSet::new();
    let mut to_delete = Vec::new();
    //
    let s = fs::read_to_string("inputfiles/day21/input.txt").unwrap();
    for l in s.lines() {
        if let Some(c) = REG.captures(&l) {
            ings.clear();
            ings.extend(c.get(1).unwrap().as_str().split(' '));
            //
            for allergen in c.get(2).unwrap().as_str().split(", ") {
                if let Some(x) = sudoku.get_mut(allergen) {
                    to_delete.clear();
                    for &x in x.iter() {
                        if !ings.contains(x) {
                            to_delete.push(x);
                        }
                    }
                    for y in &to_delete {
                        x.remove(y);
                    }
                } else {
                    sudoku.insert(allergen, ings.clone());
                }
            }
        }
    }
    //
    let mut mapping = Vec::with_capacity(sudoku.len());
    //
    while sudoku.len() > 0 {
        let mut to_be_removed = None;
        for (&allergen, ings) in &sudoku {
            if ings.len() == 1 {
                let &ing = ings.iter().next().unwrap();
                //
                to_be_removed = Some((allergen, ing));
                mapping.push((allergen, ing));
                break;
            }
        }
        if let Some((allergen, ing)) = to_be_removed {
            sudoku.remove(allergen);
            for ings in sudoku.values_mut() {
                ings.remove(ing);
            }
        }
    }
    //
    mapping.sort_by_key(|&(allergen, _)| allergen);
    //
    let ans = mapping.into_iter().map(|(_, ing)| ing).join(",");
    //
    println!("{}", ans);
}
