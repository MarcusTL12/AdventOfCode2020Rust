use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub const PARTS: [fn(); 2] = [part1, part2];

fn load_input(filename: &str) -> Vec<Vec<bool>> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().map(|x| x == '#').collect())
        .collect()
}

fn count_trees(
    forest: &Vec<Vec<bool>>,
    (speed_y, speed_x): (usize, usize),
) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    loop {
        x = (x + speed_x) % forest[0].len();
        y += speed_y;
        if y >= forest.len() {
            break;
        }
        trees += if forest[y][x] { 1 } else { 0 };
    }
    trees
}

fn part1() {
    let inp = load_input("inputfiles/day3/input.txt");
    //
    println!("{}", count_trees(&inp, (1, 3)));
}

fn part2() {
    let inp = load_input("inputfiles/day3/input.txt");
    //
    let speeds = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    //
    let ans: usize = speeds
        .iter()
        .map(|&(y, x)| count_trees(&inp, (y, x)))
        .product();
    //
    println!("{}", ans);
}
