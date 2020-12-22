use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use lazy_static::*;
use regex::Regex;

lazy_static! {
    static ref REG: Regex = Regex::new(r"Player \d:\s((?:\d+\s?)+)").unwrap();
}

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> (Vec<usize>, Vec<usize>) {
    let s = fs::read_to_string(filename).unwrap();
    //
    let mut iter = REG
        .captures_iter(&s)
        .map(|c| c[1].lines().map(|l| l.parse().unwrap()).collect());
    //
    (iter.next().unwrap(), iter.next().unwrap())
}

fn part1() {
    let (player, crab) = load_input("inputfiles/day22/input.txt");
    //
    let mut player = VecDeque::from(player);
    let mut crab = VecDeque::from(crab);
    //
    while player.len() > 0 && crab.len() > 0 {
        let p_card = player.pop_front().unwrap();
        let c_card = crab.pop_front().unwrap();
        //
        if p_card > c_card {
            player.push_back(p_card);
            player.push_back(c_card);
        } else if c_card > p_card {
            crab.push_back(c_card);
            crab.push_back(p_card);
        } else {
            unreachable!()
        }
    }
    //
    let winner = if player.len() > 0 { player } else { crab };
    //
    let score: usize = winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum();
    //
    println!("{}", score);
}

fn combat(player: &mut VecDeque<usize>, crab: &mut VecDeque<usize>) -> bool {
    let mut cache = HashSet::new();
    //
    while player.len() > 0 && crab.len() > 0 {
        if cache.contains(&(player.clone(), crab.clone())) {
            return true;
        }
        //
        cache.insert((player.clone(), crab.clone()));
        // player.hash()
        let p_card = player.pop_front().unwrap();
        let c_card = crab.pop_front().unwrap();
        //
        let winner = if player.len() >= p_card && crab.len() >= c_card {
            let mut nplayer = (0..p_card).map(|i| player[i]).collect();
            let mut ncrab = (0..c_card).map(|i| crab[i]).collect();
            combat(&mut nplayer, &mut ncrab)
        } else {
            p_card > c_card
        };
        //
        if winner {
            player.push_back(p_card);
            player.push_back(c_card);
        } else {
            crab.push_back(c_card);
            crab.push_back(p_card);
        }
    }
    //
    player.len() > 0
}

fn part2() {
    let (player, crab) = load_input("inputfiles/day22/input.txt");
    //
    let mut player = VecDeque::from(player);
    let mut crab = VecDeque::from(crab);
    //
    let winner = combat(&mut player, &mut crab);
    //
    let winner = if winner { player } else { crab };
    //
    let score: usize = winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum();
    //
    println!("{}", score);
}

