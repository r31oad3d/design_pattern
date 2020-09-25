pub trait Shape {
    fn draw(&self);
}

pub struct Rectangle {}

impl Shape for Rectangle {
    fn draw(&self) {
        println!("Rectangle::draw()");
    }
}

pub struct Square {}

impl Shape for Square {
    fn draw(&self) {
        println!("Square::draw()");
    }
}

pub struct Circle {}

impl Shape for Circle {
    fn draw(&self) {
        println!("Circle::draw()");
    }
}

pub struct ShapeMaker {
    circle: Box<dyn Shape>,
    square: Box<dyn Shape>,
    rectangle: Box<dyn Shape>,
}

impl Default for ShapeMaker {
    fn default() -> Self {
        ShapeMaker {
            circle: Box::new(Circle {}),
            square: Box::new(Square {}),
            rectangle: Box::new(Rectangle {}),
        }
    }
}

impl ShapeMaker {
    pub fn new() -> ShapeMaker {
        ShapeMaker {
            circle: Box::new(Circle {}),
            square: Box::new(Square {}),
            rectangle: Box::new(Rectangle {}),
        }
    }

    pub fn draw_circle(&self) {
        self.circle.draw();
    }

    pub fn draw_square(&self) {
        self.square.draw();
    }

    pub fn draw_rectangle(&self) {
        self.rectangle.draw();
    }
}
