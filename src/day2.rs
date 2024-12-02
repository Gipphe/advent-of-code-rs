use crate::day::Day;

pub struct Day2;

impl Day for Day2 {
    fn day_of_month(&self) -> i32 {
        2
    }

    fn task1(&self) -> i32 {
        parse_input(self.input())
            .iter()
            .filter(|x| satisfies_stable_conditions(0, x))
            .count()
            .try_into()
            .unwrap()
    }

    fn task2(&self) -> i32 {
        parse_input(self.input())
            .iter()
            .filter(|x| satisfies_stable_conditions(1, x))
            .count()
            .try_into()
            .unwrap()
    }
}

fn parse_input(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .flat_map(|y| y.parse::<i32>())
                .collect()
        })
        .collect()
}

fn is_rising(xs: &Vec<i32>) -> i32 {
    is_going_somewhere(|x, y| x > y, std::i32::MIN, xs)
}

fn is_sinking(xs: &Vec<i32>) -> i32 {
    is_going_somewhere(|x, y| x < y, std::i32::MAX, xs)
}

fn is_going_somewhere(cmp: fn(x: i32, y: i32) -> bool, start: i32, xs: &Vec<i32>) -> i32 {
    let (_, failed_levels) = xs.iter().fold((start, 0), |(v, failed_levels), x| {
        if cmp(*x, v) {
            return (*x, failed_levels);
        }
        return (v, failed_levels + 1);
    });
    failed_levels
}

fn has_smooth_gradient(xs: &Vec<i32>) -> i32 {
    let (_, failures, _) = xs
        .iter()
        .fold((false, 0, 0), |(has_started, failures, y), x| {
            if !has_started {
                return (true, failures, *x);
            }
            let diff = (x - y).abs();
            if diff >= 1 && diff <= 3 {
                return (has_started, failures, *x);
            }
            (has_started, failures + 1, y)
        });
    failures
}

fn satisfies_stable_conditions(tolerance: i32, xs: &Vec<i32>) -> bool {
    let rising_failures = is_rising(xs);
    let sinking_failures = is_sinking(xs);
    let smooth_gradient_failures = has_smooth_gradient(xs);
    (rising_failures <= tolerance || sinking_failures <= tolerance)
        && smooth_gradient_failures <= tolerance
}
