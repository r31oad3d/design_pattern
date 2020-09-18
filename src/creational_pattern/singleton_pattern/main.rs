use design_pattern::creational_pattern::singleton_pattern::{
    SingleObject, SINGLE_OBJECT_INSTANCES, SINGLE_OBJECT_INSTANCES_LOCK,
};
use std::sync::MutexGuard;

fn main() {
    {
        let obj1: MutexGuard<SingleObject> =
            SINGLE_OBJECT_INSTANCES_LOCK.lock().unwrap();
        obj1.do_it();
    }
    let obj2_temp = SINGLE_OBJECT_INSTANCES_LOCK.clone();
    let obj2 = obj2_temp.lock().unwrap();
    obj2.do_it();
    obj2;

    let obj3 = SINGLE_OBJECT_INSTANCES.clone();
    let obj4 = SINGLE_OBJECT_INSTANCES.clone();
    obj3.do_it();
    obj4.do_it();

    let obj5 = SingleObject::new();
    obj5;
    println!("THE END!");

    let obj6 = SingleObject::get_instance();
    obj6.do_it();
    let obj7 = SingleObject::get_instance();
    obj7.do_it();
}
