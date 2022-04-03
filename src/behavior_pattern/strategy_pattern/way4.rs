pub enum Strategy {
    OperationAdd,
    OperationSubtract,
    OperationMultiply,
}

impl Strategy {
    fn do_operation(&self) -> impl Fn(i32, i32) -> i32 {
        match self {
            Strategy::OperationAdd => |num1, num2| num1 + num2,
            Strategy::OperationSubtract => |num1, num2| num1 - num2,
            Strategy::OperationMultiply => |num1, num2| num1 * num2,
        }
    }
}

pub struct Context {
    strategy: Strategy,
}

impl Context {
    pub fn new(strategy: Strategy) -> Self {
        Context { strategy }
    }

    pub fn execute_strategy(&self, num1: i32, num2: i32) -> i32 {
        self.strategy.do_operation()(num1, num2)
    }
}
