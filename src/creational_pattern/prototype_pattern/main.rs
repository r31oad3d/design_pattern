use design_pattern::creational_pattern::prototype_pattern::way1::{Shape, ShapeCache};

fn main() {
    let circle: Box<dyn Shape> = ShapeCache::get_shape("1").unwrap();
    println!("{:?}", circle.get_id());
    circle.do_draw();
    println!("{:?}", circle.get_type());
    println!();

    let square: Box<dyn Shape> = ShapeCache::get_shape("2").unwrap();
    println!("{:?}", square.get_id());
    square.do_draw();
    println!("{:?}", square.get_type());
    println!();

    let rectangle: Box<dyn Shape> = ShapeCache::get_shape("3").unwrap();
    println!("{:?}", rectangle.get_id());
    rectangle.do_draw();
    println!("{:?}", rectangle.get_type());
    println!();
}
