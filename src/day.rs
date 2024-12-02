use std::fs;

pub trait Day {
    fn use_example(&self) -> bool {
        false
    }
    fn day_of_month(&self) -> i32;

    fn input(&self) -> String {
        let d = self.day_of_month();
        let ex = if self.use_example() { ".example" } else { "" };
        fs::read_to_string(format!("inputs/day{}{}.txt", d, ex))
            .unwrap()
            .trim()
            .to_string()
    }

    fn task1(&self) -> i32 {
        0
    }
    fn task2(&self) -> i32 {
        0
    }

    fn run_task1(&self) {
        let res = self.task1();
        println!("Day {} task 1 result: {}", self.day_of_month(), res);
    }

    fn run_task2(&self) {
        let res = self.task2();
        println!("Day {} task 2 result: {}", self.day_of_month(), res);
    }

    fn run_day(&self) {
        self.run_task1();
        self.run_task2();
    }
}
