#![allow(unused_imports)]
#![allow(unused_mut)]
use design_pattern::structural_pattern::flyweight_pattern::way1::{
    Circle, Shape, ShapeFactory, CIRCLE_MAP,
};
use rand::prelude::*;
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

fn main() {
    // CIRCLE_MAP
    //     .set(HashMap::<&'static str, Arc<Mutex<dyn Shape>>>::new())
    //     .unwrap();
    static COLORS: [&str; 6] =
        ["Red", "Blue", "White", "Green", "Black", "Orange"];
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        let roll1 = rng.gen_range(0, 6);
        let mut circle = ShapeFactory::get_shape(COLORS[roll1]);
        {
            let mut circle_temp1 = circle.lock().unwrap();
            let mut my_circle =
                circle_temp1.as_any_mut().downcast_mut::<Circle>().unwrap();
            let mut roll2 = rng.gen_range(0, 100);
            my_circle.set_x(roll2);
            roll2 = rng.gen_range(0, 100);
            Circle::set_y(my_circle, roll2);
            roll2 = rng.gen_range(0, 100);
            Circle::set_radius(my_circle, roll2);
        }

        circle.lock().unwrap().draw();
    }

    {
        println!("END:\n {:?}", CIRCLE_MAP.read().unwrap().keys());
    }
    {
        for e in CIRCLE_MAP.read().unwrap().values() {
            println!("{}", Arc::strong_count(e));
        }
    }
}
