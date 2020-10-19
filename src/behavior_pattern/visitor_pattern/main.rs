fn main() {
    mod way1 {
        use design_pattern::behavior_pattern::visitor_pattern::way1::{
            Computer, ComputerPart, ComputerPartDisplayVisitor,
        };

        pub fn way1() {
            let computer = Computer::new();
            let visitor = ComputerPartDisplayVisitor;
            computer.accept(&visitor)
        }
    }
    mod way2 {
        use design_pattern::behavior_pattern::visitor_pattern::way2::{
            Computer, ComputerPart, ComputerPartDisplayVisitor,
        };

        pub fn way2() {
            let computer = Computer::new();
            let visitor = ComputerPartDisplayVisitor;
            computer.accept(&visitor)
        }
    }

    println!("Way1");
    way1::way1();

    println!("Way2");
    way2::way2();
}
