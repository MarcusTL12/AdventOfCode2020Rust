use std::collections::HashMap;

pub const PARTS: [fn(); 2] = [part1, part2];

struct MemoryIter {
    memory: HashMap<u32, (u32, u32)>,
    lastnum: u32,
    i: u32,
}

impl Iterator for MemoryIter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let n = if let Some(&(amt, last)) = self.memory.get(&self.lastnum) {
            if amt == 1 {
                0
            } else {
                self.i - 1 - last
            }
        } else {
            unreachable!()
        };
        //
        if let Some((_, x)) = self.memory.get_mut(&self.lastnum) {
            *x = self.i - 1;
        }
        //
        if let Some((x, _)) = self.memory.get_mut(&n) {
            *x += 1;
        } else {
            self.memory.insert(n, (1, self.i));
        }
        //
        self.lastnum = n;
        self.i += 1;
        //
        Some(n)
    }
}

impl MemoryIter {
    fn new(starting_nums: Vec<u32>) -> impl Iterator<Item = u32> {
        let memory = starting_nums
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, (1, i as u32)))
            .collect();
        //
        let lastnum = starting_nums[starting_nums.len() - 1];
        let i = starting_nums.len() as u32;
        //
        starting_nums.into_iter().chain(Self { memory, lastnum, i })
    }
}

fn part1() {
    let ans = MemoryIter::new(vec![14, 8, 16, 0, 1, 17])
        .skip(2020 - 1)
        .next();
    //
    println!("{:?}", ans);
}

fn part2() {
    let ans = MemoryIter::new(vec![14, 8, 16, 0, 1, 17])
        .skip(30000000 - 1)
        .next();
    //
    println!("{:?}", ans);
}
