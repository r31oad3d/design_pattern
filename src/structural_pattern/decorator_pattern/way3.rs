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

pub trait ShapeDecorator : Shape
{
    fn set_red_border(&self);
}

#[derive(Default)]
pub struct RedShapeDecorator<T>
    where
        T: Shape,
{
    decorated_shape: T,
}

impl<T> Shape for RedShapeDecorator<T> where T: Shape {
    fn draw(&self) {
        self.decorated_shape.draw();
        self.set_red_border();
    }
}

impl<T> ShapeDecorator for RedShapeDecorator<T>
    where
        T: Shape,
{

    fn set_red_border(&self) {
        println!("Border Color: Red")
    }
}

impl<T> RedShapeDecorator<T>
    where
        T: Shape,
{
    pub fn new(decorated_shape: T) -> RedShapeDecorator<T> {
        RedShapeDecorator { decorated_shape }
    }


}
