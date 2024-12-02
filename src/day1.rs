use crate::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day1;

impl Day for Day1 {
    fn day_of_month(&self) -> i32 {
        1
    }

    fn task1(&self) -> i32 {
        let input = self.input();
        let (left, right) = parse_input(input);
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
            .zip(right.iter())
            .map(|(x, y)| (x - y).abs())
            .sum();
        res
    }

    fn task2(&self) -> i32 {
        let input = self.input();
        let (left, right) = parse_input(input);
        let uniques = left.into_iter().collect::<HashSet<_>>();
        let map: HashMap<i32, i32> = HashMap::new();
        let rights = right.into_iter().fold(map, |mut acc, x| {
            acc.insert(x, acc.get(&x).unwrap_or(&0) + 1);
            acc
        });
        uniques
            .iter()
            .map(|x| x * rights.get(&x).unwrap_or(&0))
            .sum()
    }
}

fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
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
        .unzip()
}
