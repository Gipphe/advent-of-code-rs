pub trait Day {
    fn day_of_month(&self) -> i32;
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
