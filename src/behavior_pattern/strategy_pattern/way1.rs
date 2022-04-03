pub trait Strategy {
    fn do_operation(&self, num1: i32, num2: i32) -> i32;
}

pub struct OperationAdd;
pub struct OperationSubtract;
pub struct OperationMultiply;
pub struct Context {
    strategy: Box<dyn Strategy>,
}

impl Strategy for OperationAdd {
    fn do_operation(&self, num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
}

impl Strategy for OperationSubtract {
    fn do_operation(&self, num1: i32, num2: i32) -> i32 {
        num1 - num2
    }
}

impl Strategy for OperationMultiply {
    fn do_operation(&self, num1: i32, num2: i32) -> i32 {
        num1 * num2
    }
}

impl Context {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Context { strategy }
    }

    pub fn execute_strategy(&self, num1: i32, num2: i32) -> i32 {
        self.strategy.do_operation(num1, num2)
    }
}
