//generic way would make Context different type
pub struct Context {
    strategy: Box<dyn Fn(i32, i32) -> i32>,
}

impl Context {
    pub fn new(strategy: Box<dyn Fn(i32, i32) -> i32>) -> Self {
        Context { strategy }
    }

    pub fn execute_strategy(&self, num1: i32, num2: i32) -> i32 {
        (self.strategy)(num1, num2)
    }
}
