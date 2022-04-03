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

pub trait Color {
    fn fill(&self);
}

pub struct Red;

pub struct Green;

pub struct Blue;

impl Color for Red {
    fn fill(&self) {
        println!("Inside Red::fill() method.");
    }
}

impl Red {
    fn new() -> Self {
        println!("Red created.");
        Red
    }
}

impl Color for Green {
    fn fill(&self) {
        println!("Inside Green::fill() method.");
    }
}

impl Green {
    fn new() -> Self {
        println!("Green created.");
        Green
    }
}

impl Color for Blue {
    fn fill(&self) {
        println!("Inside Blue::fill() method.");
    }
}

impl Blue {
    fn new() -> Self {
        println!("Blue created.");
        Blue
    }
}

pub enum Shapes {
    Rectangle,
    Square,
    Circle,
}

pub enum Colors {
    Red,
    Blue,
    Green,
}

pub enum Category {
    Shapes,
    Colors,
}

pub trait AbstractFactory {
    fn get_color(&self, color: Colors) -> Box<dyn Color>;
    fn get_shape(&self, shape: Shapes) -> Box<dyn Shape>;
}

pub struct ShapeFactory;

pub struct ColorFactory;

impl AbstractFactory for ShapeFactory {
    fn get_color(&self, _color: Colors) -> Box<dyn Color> {
        panic!("can not get Color from ShapeFactory")
    }

    fn get_shape(&self, shape: Shapes) -> Box<dyn Shape> {
        match shape {
            Shapes::Circle => Box::new(Circle::new()),
            Shapes::Rectangle => Box::new(Rectangle::new()),
            Shapes::Square => Box::new(Square::new()),
        }
    }
}

impl ShapeFactory {
    fn new() -> Self {
        println!("ShapeFactory created.");
        ShapeFactory
    }
}

impl AbstractFactory for ColorFactory {
    fn get_color(&self, color: Colors) -> Box<dyn Color> {
        match color {
            Colors::Red => Box::new(Red::new()),
            Colors::Green => Box::new(Green::new()),
            Colors::Blue => Box::new(Blue::new()),
        }
    }

    fn get_shape(&self, _shape: Shapes) -> Box<dyn Shape> {
        panic!("can not get Shape from ColorFactory")
    }
}

impl ColorFactory {
    fn new() -> Self {
        println!("ColorFactory created.");
        ColorFactory
    }
}

pub struct FactoryProducer;

impl FactoryProducer {
    pub fn get_factory(category: Category) -> Box<dyn AbstractFactory> {
        match category {
            Category::Shapes => Box::new(ShapeFactory::new()),
            Category::Colors => Box::new(ColorFactory::new()),
        }
    }
}
