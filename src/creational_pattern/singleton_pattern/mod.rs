use std::sync::{Arc, Mutex};

//Rc<>, Arc<RwLock<>>, Arc<Mutex<>>
lazy_static! {
    pub static ref SINGLE_OBJECT_INSTANCES_LOCK: Arc<Mutex<SingleObject>> =
        Arc::new(Mutex::new(SingleObject::new()));
    pub static ref SINGLE_OBJECT_INSTANCES: Arc<SingleObject> =
        Arc::new(SingleObject::new());
}

#[derive(Default)]
pub struct SingleObject;

impl SingleObject {
    pub fn new() -> Self {
        println!("SingleObject created");
        SingleObject
    }

    pub fn do_it(&self) {
        println!("SingleObject do it!")
    }

    pub fn get_instance() -> Arc<SingleObject> {
        static mut SINGLE_OBJECT_INSTANCES_UNSAFE: Option<Arc<SingleObject>> =
            None;
        unsafe {
            SINGLE_OBJECT_INSTANCES_UNSAFE
                .get_or_insert_with(|| Arc::new(SingleObject::new()))
                .clone()
        }
    }
}

impl Drop for SingleObject {
    fn drop(&mut self) {
        println!("SingleObject dropped!");
    }
}
