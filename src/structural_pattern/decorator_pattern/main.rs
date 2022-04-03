fn main() {
    mod way1 {
        use design_pattern::structural_pattern::decorator_pattern::way1::{
            Circle, Rectangle, RedShapeDecorator, Shape, ShapeDecorator,
        };

        pub fn way1() {
            let circle = Circle {};
            let red_circle = RedShapeDecorator::new(Circle {});
            let red_rectangle = RedShapeDecorator::new(Rectangle {});

            println!("Circle with normal border");
            circle.draw();
            println!("Circle with red border");
            red_circle.draw();
            println!("Rectangle with red border");
            red_rectangle.draw();
        }
    }
    println!("way1way1");
    way1::way1();

    mod way2 {
        use design_pattern::structural_pattern::decorator_pattern::way2::{
            Circle, Rectangle, RedShapeDecorator, Shape,
        };

        pub fn way2() {
            let circle = Circle {};
            //api changed
            let red_circle = RedShapeDecorator::new(Box::new(Circle {}));
            //api changed
            let red_rectangle = RedShapeDecorator::new(Box::new(Rectangle {}));

            println!("Circle with normal border");
            circle.draw();
            println!("Circle with red border");
            red_circle.draw();
            println!("Rectangle with red border");
            red_rectangle.draw();
        }
    }
    println!("way2way2");
    way2::way2();

    mod way3 {
        use design_pattern::structural_pattern::decorator_pattern::way3::{
            Circle, Rectangle, RedShapeDecorator, Shape, ShapeDecorator,
        };

        pub fn way3() {
            let circle = Circle {};
            let red_circle = RedShapeDecorator::new(Circle {});
            let red_rectangle = RedShapeDecorator::new(Rectangle {});

            println!("Circle with normal border");
            circle.draw();
            println!("Circle with red border");
            red_circle.draw();
            println!("Rectangle with red border");
            red_rectangle.draw();
        }
    }
    println!("way3way3");
    way3::way3();
}
