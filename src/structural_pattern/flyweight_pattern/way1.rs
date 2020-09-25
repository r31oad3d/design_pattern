use mut_static::MutStatic;
use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
lazy_static! {
    pub static ref CIRCLE_MAP: MutStatic<HashMap<&'static str, Arc<Mutex<dyn Shape>>>> = {
        MutStatic::from(HashMap::<&'static str, Arc<Mutex<dyn Shape>>>::new())
    };
    //  {
    // //HashMap::<&'static str, Arc<dyn Shape>>::new()
    //     let map:MutStatic<HashMap<&'static str, Arc<dyn Shape>>> = MutStatic::new();
    //     map
    // };
}
// static CIRCLE_MAP: HashMap<&'static str, Arc<dyn Shape>> = HashMap::<&'static str, Arc<dyn Shape>>::new();

pub trait Shape: Sync + Send {
    fn draw(&self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Default)]
pub struct Circle {
    color: String,
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

impl Circle {
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn set_radius(&mut self, radius: i32) {
        self.radius = radius;
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!(
            "Circle: Draw() [Color:{}, x:{}, y:{}, radius:{}]",
            self.color, self.x, self.y, self.radius
        );
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ShapeFactory {}

impl ShapeFactory {
    pub fn get_shape(color: &'static str) -> Arc<Mutex<dyn Shape>> {
        let circle_inner = {
            let temp: mut_static::ForceSomeRwLockReadGuard<
                HashMap<_, Arc<Mutex<dyn Shape>>>,
            > = CIRCLE_MAP.read().unwrap();
            let temp2 = temp.get(color);

            if let Some(circle) = temp2 {
                Some(circle.clone())
            } else {
                None
            }
        };
        if let Some(circle) = circle_inner {
            circle.clone()
        } else {
            let mut write_handler = CIRCLE_MAP.write().unwrap();
            let circle_temp = Arc::new(Mutex::new(Circle {
                color: String::from(color),
                ..Default::default()
            }));
            write_handler.insert(color, circle_temp.clone());
            circle_temp
        }
    }
}
