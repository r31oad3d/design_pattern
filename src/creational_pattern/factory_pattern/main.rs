use design_pattern::creational_pattern::factory_pattern::ShapeFactory;

fn main() {
    let shape1 = ShapeFactory::get_shape("circle");
    if let Some(shape) = shape1 {
        shape.draw()
    } else {
        println!("Not get valid shape");
    };

    let shape1 = ShapeFactory::get_shape("rectangle");
    if let Some(shape) = shape1 {
        shape.draw()
    } else {
        println!("Not get valid shape");
    }

    let shape1 = ShapeFactory::get_shape("Square");
    if let Some(shape) = shape1 {
        shape.draw()
    } else {
        println!("Not get valid shape");
    };

    let shape1 = ShapeFactory::get_shape("xxx");
    if let Some(shape) = shape1 {
        shape.draw()
    } else {
        println!("Not get valid shape");
    };
}
