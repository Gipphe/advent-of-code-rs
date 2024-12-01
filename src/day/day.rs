mod day1;

pub type TaskFn = fn() -> Result<i32, String>;

pub trait Day {
    fn day_of_month(&self) -> i32;
    fn task1(&self) -> i32;
    fn task2(&self) -> i32;
}

impl Day {
    fn input(&self) -> &str {}
    fn run_task1(&self) {
        if let res = self.task1() {
            println!("Day {} task 1 result: {}", self.day_of_month(), res)
        } else {
            println!("Day {} task 1 implementation missing", self.day_of_month())
        }
    }
    fn run_task2(&self) {
        if let res = self.task2 {
            println!("Day {} task 2 result: {}", self.day_of_month(), res)
        } else {
            println!("Day {} task 2 implementation missing", self.day_of_month)
        }
    }
    pub fn run_day(&self) {
        self.run_task1();
        self.run_task2();
    }
}
