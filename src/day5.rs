use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use arrayvec::ArrayVec;

pub const PARTS: [fn(); 2] = [part1, part2];

fn board_id(board: &str) -> Option<u32> {
    board
        .chars()
        .map(|c| match c {
            'F' | 'L' => b'0',
            'B' | 'R' => b'1',
            _ => unreachable!(),
        })
        .collect::<ArrayVec<[_; 10]>>()
        .into_inner()
        .ok()
        .and_then(|x| {
            std::str::from_utf8(&x)
                .ok()
                .and_then(|s| u32::from_str_radix(s, 2).ok())
        })
}

fn part1() {
    let ans = BufReader::new(File::open("inputfiles/day5/input.txt").unwrap())
        .lines()
        .filter_map(|l| l.ok().and_then(|l| board_id(&l)))
        .max();
    //
    println!("{:?}", ans);
}

fn part2() {
    let boards: HashSet<_> =
        BufReader::new(File::open("inputfiles/day5/input.txt").unwrap())
            .lines()
            .filter_map(|l| l.ok().and_then(|l| board_id(&l)))
            .collect();
    //
    let ans = boards
        .iter()
        .min()
        .and_then(|&m| (m..).filter(|i| !boards.contains(i)).next());
    //
    println!("{:?}", ans);
}
