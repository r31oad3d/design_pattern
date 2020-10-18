fn main() {
    mod way1 {
        use design_pattern::behavior_pattern::strategy_pattern::way1::{
            Context, OperationAdd, OperationMultiply, OperationSubtract,
        };

        pub fn way1() {
            let mut context = Context::new(Box::new(OperationAdd));
            println!("10 + 5 = {}", context.execute_strategy(10, 5));

            context = Context::new(Box::new(OperationSubtract));
            println!("10 - 5 = {}", context.execute_strategy(10, 5));

            context = Context::new(Box::new(OperationMultiply));
            println!("10 * 5 = {}", context.execute_strategy(10, 5));
        }
    }

    mod way2 {
        use design_pattern::behavior_pattern::strategy_pattern::way2::{
            Context, Strategy,
        };

        pub fn way2() {
            let mut context = Context::new(Strategy::OperationAdd);
            println!("10 + 5 = {}", context.execute_strategy(10, 5));

            context = Context::new(Strategy::OperationSubtract);
            println!("10 - 5 = {}", context.execute_strategy(10, 5));

            context = Context::new(Strategy::OperationMultiply);
            println!("10 * 5 = {}", context.execute_strategy(10, 5));
        }
    }

    mod way3 {
        use design_pattern::behavior_pattern::strategy_pattern::way3::Context;

        pub fn way3() {
            let mut context = Context::new(Box::new(|num1, num2| num1 + num2));
            println!("10 + 5 = {}", context.execute_strategy(10, 5));

            context = Context::new(Box::new(|num1, num2| num1 - num2));
            println!("10 - 5 = {}", context.execute_strategy(10, 5));

            context = Context::new(Box::new(|num1, num2| num1 * num2));
            println!("10 * 5 = {}", context.execute_strategy(10, 5));
        }
    }

    mod way4 {
        use design_pattern::behavior_pattern::strategy_pattern::way4::{
            Context, Strategy,
        };

        pub fn way4() {
            let mut context = Context::new(Strategy::OperationAdd);
            println!("10 + 5 = {}", context.execute_strategy(10, 5));

            context = Context::new(Strategy::OperationSubtract);
            println!("10 - 5 = {}", context.execute_strategy(10, 5));

            context = Context::new(Strategy::OperationMultiply);
            println!("10 * 5 = {}", context.execute_strategy(10, 5));
        }
    }

    println!("Way1:");
    way1::way1();

    println!("Way2:");
    way2::way2();

    println!("Way3:");
    way3::way3();

    println!("Way4:");
    way4::way4();
}
