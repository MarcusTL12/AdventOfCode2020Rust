use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Peekable,
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn eager_eval<I: Iterator<Item = char>>(
    s: &mut Peekable<I>,
    buf: &mut String,
) -> u64 {
    let mut acc = 0;
    let mut op = false;
    //
    while let Some(_) = s.peek() {
        while let Some(' ') = s.peek() {
            s.next();
        }
        //
        match s.peek() {
            Some(&c) => match c {
                '(' | ')' | '+' | '*' => {
                    s.next();
                    match c {
                        '(' => {
                            let num = eager_eval(s, buf);
                            if op {
                                acc *= num;
                            } else {
                                acc += num;
                            }
                        }
                        ')' => return acc,
                        '+' => op = false,
                        '*' => op = true,
                        _ => unreachable!(),
                    }
                }
                _ => {
                    buf.clear();
                    while let Some('0'..='9') = s.peek() {
                        buf.push(s.next().unwrap());
                    }
                    if let Ok(num) = buf.parse::<u64>() {
                        if op {
                            acc *= num;
                        } else {
                            acc += num;
                        }
                    }
                }
            },
            None => (),
        }
    }
    //
    acc
}

fn part1() {
    let mut buf = String::new();
    //
    let ans: u64 =
        BufReader::new(File::open("inputfiles/day18/input.txt").unwrap())
            .lines()
            .map(|l| eager_eval(&mut l.unwrap().chars().peekable(), &mut buf))
            .sum();
    //
    println!("{}", ans);
}

fn opp_eval<I: Iterator<Item = char>>(
    s: &mut Peekable<I>,
    buf: &mut String,
) -> u64 {
    let mut acc = 1;
    let mut add_acc = 0;
    //
    while let Some(_) = s.peek() {
        while let Some(' ') = s.peek() {
            s.next();
        }
        //
        match s.peek() {
            Some(&c) => match c {
                '(' | ')' | '+' | '*' => {
                    s.next();
                    match c {
                        '(' => {
                            let num = opp_eval(s, buf);
                            add_acc += num;
                        }
                        ')' => return acc * add_acc,
                        '*' => {
                            acc *= add_acc;
                            add_acc = 0
                        }
                        '+' => (),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    buf.clear();
                    while let Some('0'..='9') = s.peek() {
                        buf.push(s.next().unwrap());
                    }
                    if let Ok(num) = buf.parse::<u64>() {
                        add_acc += num;
                    }
                }
            },
            None => (),
        }
    }
    //
    acc * add_acc
}

fn part2() {
    let mut buf = String::new();
    //
    let ans: u64 =
        BufReader::new(File::open("inputfiles/day18/input.txt").unwrap())
            .lines()
            .map(|l| opp_eval(&mut l.unwrap().chars().peekable(), &mut buf))
            .sum();
    //
    println!("{}", ans);
}
