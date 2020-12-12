use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

#[derive(Debug, Clone, PartialEq, Eq)]
enum Seat {
    Empty,
    Occupied,
}

const DIRS: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn load_input(filename: &str) -> Vec<Vec<Option<Seat>>> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'L' => Some(Seat::Empty),
                    '.' => None,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn do_step(
    from: &Vec<Vec<Option<Seat>>>,
    to: &mut Vec<Vec<Option<Seat>>>,
    amt_neighbours: usize,
    neighbour_counter: fn(&Vec<Vec<Option<Seat>>>, (usize, usize)) -> usize,
) -> bool {
    let h = from.len();
    let w = from[0].len();
    //
    let mut did_something = false;
    //
    for i in 0..h {
        for j in 0..w {
            let neighbours = neighbour_counter(from, (i, j));
            to[i][j] = match from[i][j] {
                Some(Seat::Empty) => {
                    if neighbours == 0 {
                        Some(Seat::Occupied)
                    } else {
                        Some(Seat::Empty)
                    }
                }
                Some(Seat::Occupied) => {
                    if neighbours >= amt_neighbours {
                        Some(Seat::Empty)
                    } else {
                        Some(Seat::Occupied)
                    }
                }
                None => None,
            };
            //
            if to[i][j] != from[i][j] {
                did_something = true;
            }
        }
    }
    //
    did_something
}

fn part1() {
    let mut inp = load_input("inputfiles/day11/input.txt");
    //
    let mut buf = inp.clone();
    //
    let mut a = &mut inp;
    let mut b = &mut buf;
    //
    while do_step(a, b, 4, |map, (i, j)| {
        DIRS.iter()
            .filter_map(|(a, b)| {
                map.get((i as isize + a) as usize)
                    .and_then(|row| row.get((j as isize + b) as usize))
                    .and_then(|x| x.as_ref())
            })
            .map(|x| match x {
                Seat::Empty => 0,
                Seat::Occupied => 1,
            })
            .sum()
    }) {
        let tmp = a;
        a = b;
        b = tmp;
    }
    //
    let ans: usize = a
        .iter()
        .flat_map(|x| x.iter())
        .filter_map(|x| x.as_ref())
        .map(|x| match x {
            Seat::Occupied => 1,
            Seat::Empty => 0,
        })
        .sum();
    //
    println!("{}", ans);
}

fn straight_line_neighbours(
    map: &Vec<Vec<Option<Seat>>>,
    (i, j): (usize, usize),
) -> usize {
    DIRS.iter()
        .map(|(a, b)| {
            match (1..)
                .map(|k| {
                    (
                        (a * k + i as isize) as usize,
                        (b * k + j as isize) as usize,
                    )
                })
                .filter_map(|(a, b)| {
                    map.get(a)
                        .and_then(|x| x.get(b))
                        .unwrap_or(&Some(Seat::Empty))
                        .as_ref()
                })
                .next()
            {
                Some(Seat::Occupied) => 1,
                Some(Seat::Empty) => 0,
                None => unreachable!(),
            }
        })
        .sum()
}

fn part2() {
    let mut inp = load_input("inputfiles/day11/input.txt");
    //
    let mut buf = inp.clone();
    //
    let mut a = &mut inp;
    let mut b = &mut buf;
    //
    while do_step(a, b, 5, straight_line_neighbours) {
        let tmp = a;
        a = b;
        b = tmp;
    }
    //
    let ans: usize = a
        .iter()
        .flat_map(|x| x.iter())
        .filter_map(|x| x.as_ref())
        .map(|x| match x {
            Seat::Occupied => 1,
            Seat::Empty => 0,
        })
        .sum();
    //
    println!("{}", ans);
}
