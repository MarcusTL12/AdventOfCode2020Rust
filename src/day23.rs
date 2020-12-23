// const INP: &[u8] = b"389125467";
const INP: &[u8] = b"326519478";

pub const PARTS: [fn(); 2] = [part1, part2];

fn init_cups<I: Iterator<Item = usize>>(
    it: I,
    size: usize,
) -> (Vec<(usize, usize)>, usize) {
    let mut cups = vec![(0, 0); size];
    //
    let mut first_node = None;
    let mut prev_node = None;
    //
    for i in it {
        let mut node = (0, 0);
        if let Some(prev_node) = prev_node {
            node.0 = prev_node;
            cups[prev_node].1 = i;
        } else {
            first_node = Some(i);
        }
        prev_node = Some(i);
        cups[i] = node;
    }
    //
    if let Some((first_node, last_node)) = first_node.and_then(|first_node| {
        prev_node.and_then(|prev_node| Some((first_node, prev_node)))
    }) {
        cups[first_node].0 = last_node;
        cups[last_node].1 = first_node;
    }
    //
    (cups, first_node.unwrap())
}

fn do_step(cups: &mut [(usize, usize)], cur_cup: &mut usize) {
    let pickup = {
        let mut slice = [0; 3];
        slice[0] = cups[*cur_cup].1;
        slice[1] = cups[slice[0]].1;
        slice[2] = cups[slice[1]].1;
        slice
    };
    //
    {
        let last_next = cups[pickup[2]].1;
        let first_prev = cups[pickup[0]].0;
        cups[first_prev].1 = last_next;
        cups[last_next].0 = first_prev;
    }
    //
    let mut dest = if *cur_cup == 0 {
        cups.len() - 1
    } else {
        *cur_cup - 1
    };
    //
    while pickup.iter().any(|&i| i == dest) {
        dest = if dest == 0 { cups.len() - 1 } else { dest - 1 };
    }
    //
    cups[pickup[2]].1 = cups[dest].1;
    cups[pickup[0]].0 = dest;
    //
    cups[dest].1 = pickup[0];
    let last_next = cups[pickup[2]].1;
    cups[last_next].0 = pickup[2];
    //
    *cur_cup = cups[*cur_cup].1;
}

fn part1() {
    let (mut cups, mut cur_cup) =
        init_cups(INP.iter().map(|i| (i - b'1') as usize), INP.len());
    //
    for _ in 0..100 {
        do_step(&mut cups, &mut cur_cup);
    }
    //
    {
        let mut node = cups[0].1;
        while node != 0 {
            print!("{}", node + 1);
            node = cups[node].1;
        }
        println!("");
    }
}

fn part2() {
    let (mut cups, mut cur_cup) = init_cups(
        INP.iter()
            .map(|i| (i - b'1') as usize)
            .chain(INP.len()..1000_000),
        1000_000,
    );
    //
    for _ in 0..10_000_000 {
        do_step(&mut cups, &mut cur_cup);
    }
    //
    let ans = (cups[0].1 + 1) * (cups[cups[0].1].1 + 1);
    //
    println!("{}", ans);
}
