pub type TaskFn = fn() -> i32;

pub struct Day {
    day_of_month: i32,
    task1: Option<TaskFn>,
    task2: Option<TaskFn>,
}

impl Day {
    fn run_task1(&self) {
        if let Some(res) = self.task1.map(|f| f()) {
            println!("Day {} task 1 result: {}", self.day_of_month, res)
        } else {
            println!("Day {} task 1 implementation missing", self.day_of_month)
        }
    }
    fn run_task2(&self) {
        if let Some(res) = self.task2.map(|f| f()) {
            println!("Day {} task 2 result: {}", self.day_of_month, res)
        } else {
            println!("Day {} task 2 implementation missing", self.day_of_month)
        }
    }
    pub fn run_day(&self) {
        self.run_task1();
        self.run_task2();
    }
}
