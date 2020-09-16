pub trait Shape {
    fn draw(&self);
}

pub struct Rectangle;

pub struct Square;

pub struct Circle;

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Inside Rectangle::draw() method.");
    }
}

impl Rectangle {
    fn new() -> Self {
        println!("Rectangle created.");
        Rectangle
    }
}

impl Shape for Square {
    fn draw(&self) {
        println!("Inside Square::draw() method.");
    }
}

impl Square {
    fn new() -> Self {
        println!("Square created.");
        Square
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Inside Circle::draw() method.");
    }
}

impl Circle {
    fn new() -> Self {
        println!("Circle created.");
        Circle
    }
}

pub struct ShapeFactory;

const RECTANGLE: &str = "rectangle";
const SQUARE: &str = "square";
const CIRCLE: &str = "circle";
impl ShapeFactory {
    pub fn get_shape(shape_type: &str) -> Option<Box<dyn Shape>> {
        match shape_type.to_lowercase().as_str() {
            RECTANGLE => Some(Box::new(Rectangle::new())),
            SQUARE => Some(Box::new(Square::new())),
            CIRCLE => Some(Box::new(Circle::new())),
            _ => None,
        }
    }
}
