use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> (Vec<Vec<(usize, usize)>>, usize) {
    let reg1 = Regex::new(r"(.+) bags contain (.+)").unwrap();
    let reg2 = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    //
    let mut bags = HashMap::new();
    //
    fn get_bag(bag: &str, bags: &mut HashMap<String, usize>) -> usize {
        if let Some(&ind) = bags.get(bag) {
            ind
        } else {
            let ind = bags.len();
            bags.insert(bag.to_owned(), ind);
            ind
        }
    }
    //
    let mut tmp: HashMap<usize, Option<Vec<(usize, usize)>>> =
        BufReader::new(File::open(filename).unwrap())
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                if let Some(c) = reg1.captures(&l) {
                    let k = get_bag(&c[1], &mut bags);
                    //
                    let v = reg2
                        .captures_iter(&c[2])
                        .map(|c| {
                            let amt = c[1].parse().unwrap();
                            let bag = get_bag(&c[2], &mut bags);
                            (amt, bag)
                        })
                        .collect();
                    //
                    (k, Some(v))
                } else {
                    unreachable!()
                }
            })
            .collect();
    //
    (
        (0..bags.len())
            .filter_map(|k| {
                if let Some(x) = tmp.get_mut(&k) {
                    x.take()
                } else {
                    unreachable!()
                }
            })
            .collect(),
        bags["shiny gold"],
    )
}

fn has_bag(
    bag: usize,
    looking_for: usize,
    inp: &[Vec<(usize, usize)>],
    memo: &mut [Option<bool>],
) -> bool {
    if let Some(ans) = memo[bag] {
        ans
    } else {
        let tmp = inp[bag].iter().any(|&(_, x)| {
            x == looking_for || has_bag(x, looking_for, inp, memo)
        });
        memo[bag] = Some(tmp);
        tmp
    }
}

fn part1() {
    let (inp, shiny_gold) = load_input("inputfiles/day7/input.txt");
    //
    let mut memo = vec![None; inp.len()];
    //
    let ans = (0..inp.len())
        .filter(|&bag| has_bag(bag, shiny_gold, &inp, &mut memo))
        .count();
    //
    println!("{:?}", ans);
}

fn count_bags(
    bag: usize,
    inp: &[Vec<(usize, usize)>],
    memo: &mut [Option<usize>],
) -> usize {
    if let Some(ans) = memo[bag] {
        ans
    } else {
        let tmp = inp[bag]
            .iter()
            .map(|&(amt, bag)| amt * count_bags(bag, inp, memo))
            .sum::<usize>()
            + 1;
        memo[bag] = Some(tmp);
        tmp
    }
}

fn part2() {
    let (inp, shiny_gold) = load_input("inputfiles/day7/input.txt");
    //
    let mut memo = vec![None; inp.len()];
    //
    let ans = count_bags(shiny_gold, &inp, &mut memo) - 1;
    //
    println!("{:?}", ans);
}
