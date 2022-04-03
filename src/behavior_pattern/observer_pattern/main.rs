use design_pattern::behavior_pattern::observer_pattern::way1::{
    BinaryObserver, HexaObserver, OctalObserver, Subject,
};
use std::rc::Rc;

fn main() {
    let subject = Rc::new(Subject::new());
    let hexa_observer = HexaObserver::new(subject.clone());
    let octal_observer = OctalObserver::new(subject.clone());
    let binary_observer = BinaryObserver::new(subject.clone());
    subject.attach(hexa_observer);
    subject.attach(octal_observer);
    subject.attach(binary_observer);

    println!("strong ref:{}", Rc::strong_count(&subject));
    println!("weak ref:{}", Rc::weak_count(&subject));

    println!("First state change: 15");
    subject.set_state(15);
    println!("First state change: 10");
    subject.set_state(10);
}
