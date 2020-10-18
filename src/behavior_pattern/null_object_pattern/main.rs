fn main() {
    mod way1 {
        use design_pattern::behavior_pattern::null_object_pattern::way1::CustomerFactory;

        pub fn way1() {
            let customer1 = CustomerFactory::get_customer("Rob");
            let customer2 = CustomerFactory::get_customer("Bob");
            let customer3 = CustomerFactory::get_customer("Julie");
            let customer4 = CustomerFactory::get_customer("Laura");

            println!("Customers");
            println!("{}", customer1.get_name());
            println!("{}", customer2.get_name());
            println!("{}", customer3.get_name());
            println!("{}", customer4.get_name());
        }
    }

    mod way2 {
        use design_pattern::behavior_pattern::null_object_pattern::way2::{
            AbstractCustomer, CustomerFactory,
        };

        pub fn way2() {
            let customer1 = CustomerFactory::get_customer("Rob");
            let customer2 = CustomerFactory::get_customer("Bob");
            let customer3 = CustomerFactory::get_customer("Julie");
            let customer4 = CustomerFactory::get_customer("Laura");

            println!("Customers");
            println!("{}", customer1.get_name());
            println!("{}", customer2.get_name());
            println!("{}", customer3.get_name());
            println!("{}", customer4.get_name());
        }
    }

    mod way3 {
        use design_pattern::behavior_pattern::null_object_pattern::way3::{
            AbstractCustomer, CustomerFactory,
        };

        pub fn way3() {
            let customer1 = CustomerFactory::get_customer("Rob");
            let customer2 = CustomerFactory::get_customer("Bob");
            let customer3 = CustomerFactory::get_customer("Julie");
            let customer4 = CustomerFactory::get_customer("Laura");

            println!("Customers");
            println!("{}", customer1.get_name());
            println!("{}", customer2.get_name());
            println!("{}", customer3.get_name());
            println!("{}", customer4.get_name());
        }
    }

    println!("Way1:");
    way1::way1();
    println!("Way2:");
    way2::way2();
    println!("Way3:");
    way3::way3();
}
