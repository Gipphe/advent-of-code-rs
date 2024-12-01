use std::fs;

mod st;

pub struct Day1;

impl Day {
    fn input() -> &str {
        fs::read_to_string("inputs/day1.txt").unwrap()
    }
}

impl Day for Day1 {
    fn day_of_month(&self) {
        1
    }

    fn task1(&self) -> i32 {
    let lines = self.input().lines();
    let (left, right): (Vec<i32>, Vec<i32>) = lines
        .map(|x| {
            let col: Vec<_> = x.split_whitespace().collect();
            let first = col[0];
            let second = col[1];
            (first.parse::<i32>(), second.parse::<i32>())
        })
        .flat_map(|x| {
            if let (Ok(l), Ok(r)) = x {
                Some((l, r))
            } else {
                None
            }
        })
        .unzip();
    let left: Vec<i32> = {
        let mut x = left.clone();
        x.sort();
        x
    };
    let right: Vec<i32> = {
        let mut x = right.clone();
        x.sort();
        x
    };
    let res: i32 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(x, y)| (x - y).abs())
        .sum();
    println!("{}", res);
    }
}

pub const DAY: Day = Day{
    day_of_month: 1,
    task1: Some(|| {
    }),
    task2: None,
}
