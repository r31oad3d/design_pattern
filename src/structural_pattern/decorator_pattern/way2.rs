pub trait Shape {
    fn draw(&self);
}

pub struct Circle();

impl Shape for Circle {
    fn draw(&self) {
        println!("Shape: Circle")
    }
}

pub struct Rectangle();

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Shape: Rectangle")
    }
}

pub trait ShapeDecorator: Shape {
    fn set_red_border(&self);
}

pub struct RedShapeDecorator {
    decorated_shape: Box<dyn Shape>,
}

impl Shape for RedShapeDecorator {
    fn draw(&self) {
        self.decorated_shape.draw();
        self.set_red_border();
    }
}

impl ShapeDecorator for RedShapeDecorator {
    fn set_red_border(&self) {
        println!("Border Color: Red")
    }
}

impl RedShapeDecorator {
    pub fn new(decorated_shape: Box<dyn Shape>) -> RedShapeDecorator {
        RedShapeDecorator { decorated_shape }
    }
}
