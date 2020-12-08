use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug, Clone)]
enum Ins {
    Acc(i64),
    Jmp(isize),
    Nop(isize),
}

fn load_input(filename: &str) -> Vec<Ins> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(
            |l| match l.split(' ').collect::<ArrayVec<[_; 2]>>().as_slice() {
                ["acc", n] => Ins::Acc(n.parse().unwrap()),
                ["jmp", n] => Ins::Jmp(n.parse().unwrap()),
                ["nop", n] => Ins::Nop(n.parse().unwrap()),
                _ => unreachable!(),
            },
        )
        .collect()
}

fn part1() {
    let inp = load_input("inputfiles/day8/input.txt");
    //
    let mut i = 0;
    let mut visited = vec![false; inp.len()];
    let mut acc = 0;
    //
    while i < inp.len() && !visited[i] {
        visited[i] = true;
        match inp[i] {
            Ins::Acc(n) => acc += n,
            Ins::Jmp(n) => i = (i as isize + n - 1) as usize,
            Ins::Nop(_) => (),
        }
        i += 1
    }
    println!("{}", acc);
}

fn part2() {
    let inp = load_input("inputfiles/day8/input.txt");
    //
    let ans = (0..inp.len())
        .filter_map(|change_ind| {
            let mut n_inp = inp.clone();
            n_inp[change_ind] = match n_inp[change_ind] {
                Ins::Acc(_) => return None,
                Ins::Jmp(n) => Ins::Nop(n),
                Ins::Nop(n) => Ins::Jmp(n),
            };
            let mut i = 0;
            let mut visited = vec![false; inp.len()];
            let mut acc = 0;
            //
            while i < n_inp.len() && !visited[i] {
                visited[i] = true;
                match n_inp[i] {
                    Ins::Acc(n) => acc += n,
                    Ins::Jmp(n) => i = (i as isize + n - 1) as usize,
                    Ins::Nop(_) => (),
                }
                i += 1
            }
            if i >= inp.len() {
                Some(acc)
            } else {
                None
            }
        })
        .next()
        .unwrap();
    //
    println!("{:?}", ans);
}
