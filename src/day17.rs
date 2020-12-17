use std::{collections::HashSet, fs};

use itertools::Itertools;

use collect_slice::CollectSlice;

pub const PARTS: [fn(); 2] = [part1, part2];

fn count_neighbours<const N: usize>(
    map: &HashSet<[i32; N]>,
    coord: [i32; N],
) -> usize {
    (0..N)
        .map(|i| coord[i] - 1..=coord[i] + 1)
        .multi_cartesian_product()
        .filter(|ncoord| {
            let mut slice = [0; N];
            ncoord.iter().cloned().collect_slice(&mut slice);
            slice != coord && map.contains(&slice)
        })
        .count()
}

fn do_step<const N: usize>(
    from: &HashSet<[i32; N]>,
    to: &mut HashSet<[i32; N]>,
) {
    let minmax = {
        let mut slice = [(0, 0); N];
        (0..N)
            .map(|i| match from.iter().map(|x| x[i]).minmax() {
                itertools::MinMaxResult::OneElement(x) => (x - 1, x + 1),
                itertools::MinMaxResult::MinMax(x, y) => (x - 1, y + 1),
                _ => unreachable!(),
            })
            .collect_slice(&mut slice);
        slice
    };
    to.clear();
    //
    for coord in minmax.iter().map(|&(x, y)| x..=y).multi_cartesian_product() {
        let mut slice = [0; N];
        coord.into_iter().collect_slice(&mut slice);
        let neighbours = count_neighbours(&from, slice);
        if from.contains(&slice) {
            if matches!(neighbours, 2 | 3) {
                to.insert(slice);
            }
        } else {
            if neighbours == 3 {
                to.insert(slice);
            }
        }
    }
}

fn part1() {
    let mut inp: HashSet<_> = fs::read_to_string("inputfiles/day17/input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some([x as i32, y as i32, 0]),
                _ => None,
            })
        })
        .collect();
    //
    let mut other = HashSet::new();
    //
    let mut a = &mut inp;
    let mut b = &mut other;
    //
    for _ in 0..6 {
        do_step(a, b);
        let tmp = a;
        a = b;
        b = tmp;
    }
    //
    println!("{}", a.len());
}

fn part2() {
    let mut inp: HashSet<_> = fs::read_to_string("inputfiles/day17/input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars().enumerate().filter_map(move |(x, c)| match c {
                '#' => Some([x as i32, y as i32, 0, 0]),
                _ => None,
            })
        })
        .collect();
    //
    let mut other = HashSet::new();
    //
    let mut a = &mut inp;
    let mut b = &mut other;
    //
    for _ in 0..6 {
        do_step(a, b);
        let tmp = a;
        a = b;
        b = tmp;
    }
    //
    println!("{}", a.len());
}
