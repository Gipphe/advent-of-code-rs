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
        .fold((false, 0, 0), |(has_started, failures, prev), curr| {
            if !has_started {
                return (true, failures, *curr);
            }
            let diff = (curr - prev).abs();
            if diff >= 1 && diff <= 3 {
                return (has_started, failures, *curr);
            }
            (has_started, failures + 1, prev)
        });
    failures
}

fn satisfies_stable_conditions(tolerance: i32, xs: &Vec<i32>) -> bool {
    let rising_ok = is_rising(xs) <= tolerance;
    let sinking_ok = is_sinking(xs) <= tolerance;
    let smooth_gradient_ok = has_smooth_gradient(xs) <= tolerance;
    println!(
        "rising: {}, sinking: {}, smooth: {}",
        rising_ok, sinking_ok, smooth_gradient_ok
    );
    (rising_ok || sinking_ok) && smooth_gradient_ok
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn satisfies_stable_conditions_single_item() {
        let good_case: Vec<i32> = vec![50];
        let actual = satisfies_stable_conditions(1, &good_case);
        assert_eq!(actual, true);
    }

    #[test]
    fn satisfies_stable_conditions_two_items() {
        assert_eq!(satisfies_stable_conditions(1, &vec![50, 51]), true);
    }

    #[test]
    fn satisfies_stable_conditions_multiple_items() {
        assert_eq!(
            satisfies_stable_conditions(1, &vec![50, 49, 48, 45, 42]),
            true
        );
    }

    #[test]
    fn is_rising_one_item() {
        assert_eq!(is_rising(&vec![1, 2]), 0);
    }
}
