use std::cell::Cell;

pub struct SpiderRobot {
    hardware_error_count: Cell<u32>
}

impl SpiderRobot {
    pub fn new() -> SpiderRobot {
        SpiderRobot { hardware_error_count: Cell::new(0) }
    }

    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    pub fn has_hardware_errors(&self) -> bool {
        self.hardware_error_count.get() > 0
    }
}

fn main() {
    let robot = SpiderRobot::new();
    println!("{}", robot.has_hardware_errors());
    robot.add_hardware_error();
    println!("{}", robot.has_hardware_errors());
}
