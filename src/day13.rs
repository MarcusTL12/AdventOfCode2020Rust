use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use ring_algorithm::chinese_remainder_theorem;

pub const PARTS: [fn(); 2] = [part1, part2];

fn part1() {
    let (t, ids) = {
        let mut lines =
            BufReader::new(File::open("inputfiles/day13/input.txt").unwrap())
                .lines()
                .map(|l| l.unwrap());
        //
        let t: u64 = lines.next().and_then(|x| x.parse().ok()).unwrap();
        //
        let ids: Vec<u64> = lines
            .next()
            .unwrap()
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect();
        //
        (t, ids)
    };
    //
    let ans = (t..)
        .filter_map(|i| {
            ids.iter()
                .filter_map(|id| {
                    if i % id == 0 {
                        Some((i - t) * id)
                    } else {
                        None
                    }
                })
                .next()
        })
        .next();
    //
    println!("{:?}", ans);
}

fn part2() {
    let inp: Vec<(i64, i64)> =
        BufReader::new(File::open("inputfiles/day13/input.txt").unwrap())
            .lines()
            .skip(1)
            .next()
            .and_then(|l| l.ok())
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(i, x)| {
                x.parse().ok().and_then(|n| Some((i as i64, n)))
            })
            .collect();
    //
    let a: Vec<_> = inp.iter().map(|(i, _)| -i).collect();
    let m: Vec<_> = inp.iter().map(|&(_, i)| i).collect();
    //
    let ans = chinese_remainder_theorem(&a, &m);
    //
    println!("{:?}", ans);
}
