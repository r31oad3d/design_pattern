use std::ops::Deref;

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

pub trait ShapeDecorator:Deref
{
    fn draw(&self);
    fn set_red_border(&self);
}

#[derive(Default)]
pub struct RedShapeDecorator<T>
    where
        T: Shape,
{
    decorated_shape: T,
}

impl<T> ShapeDecorator for RedShapeDecorator<T>
    where
        T: Shape,
{
    fn draw(&self) {
        self.decorated_shape.draw();
        self.set_red_border();
    }
    fn set_red_border(&self) {
        println!("Border Color: Red")
    }
}

impl<T> Deref for RedShapeDecorator<T> where T: Shape{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.decorated_shape
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
