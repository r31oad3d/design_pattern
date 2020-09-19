use dyn_clone::{clone_trait_object, DynClone};
use std::collections::HashMap;
pub trait Shape: Sync + DynClone {
    fn get_id(&self) -> &String;
    fn get_type(&self) -> &String;
    fn do_draw(&self);
}
clone_trait_object!(Shape);

#[derive(Clone)]
pub struct Rectangle {
    id: String,
    shape_type: String,
}
impl Rectangle {
    pub fn new(id: String, shape_type: String) -> Self {
        Rectangle { id, shape_type }
    }
}
impl Shape for Rectangle {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_type(&self) -> &String {
        &self.shape_type
    }

    fn do_draw(&self) {
        println!("Inside Rectangle::draw() method.");
    }
}

#[derive(Clone)]
pub struct Square {
    id: String,
    shape_type: String,
}
impl Square {
    pub fn new(id: String, shape_type: String) -> Self {
        Square { id, shape_type }
    }
}
impl Shape for Square {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_type(&self) -> &String {
        &self.shape_type
    }

    fn do_draw(&self) {
        println!("Inside Square::draw() method.");
    }
}

#[derive(Clone)]
pub struct Circle {
    id: String,
    shape_type: String,
}
impl Circle {
    pub fn new(id: String, shape_type: String) -> Self {
        Circle { id, shape_type }
    }
}
impl Shape for Circle {
    fn get_id(&self) -> &String {
        &self.id
    }

    fn get_type(&self) -> &String {
        &self.shape_type
    }

    fn do_draw(&self) {
        println!("Inside Circle::draw() method.");
    }
}

lazy_static! {
    pub static ref SHAPE_CACHE: HashMap<&'static str, Box<dyn Shape>> = {
        let mut map: HashMap<&'static str, Box<dyn Shape>> = HashMap::new();
        map.insert(
            "1",
            Box::new(Circle::new("1".to_owned(), "Circle".to_owned())),
        );
        map.insert(
            "2",
            Box::new(Square::new("2".to_owned(), "Square".to_owned())),
        );
        map.insert(
            "3",
            Box::new(Rectangle::new("3".to_owned(), "Rectangle".to_owned())),
        );
        map
    };
}

pub struct ShapeCache {}
impl ShapeCache {
    pub fn get_shape(type_id: &str) -> Option<Box<dyn Shape>> {
        match SHAPE_CACHE.get(type_id) {
            Some(shape) => Some((*shape).clone()),
            None => None,
        }
    }
}
