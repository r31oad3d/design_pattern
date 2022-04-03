use design_pattern::behavior_pattern::memento_pattern::way1::{
    CareTaker, Originator,
};

fn main() {
    let mut originator = Originator::new("State #1".to_owned());
    let mut caretaker = CareTaker::new();

    originator.set_state("State #2".to_owned());
    caretaker.add(originator.save_state_to_memento());

    originator.set_state("State #3".to_owned());
    caretaker.add(originator.save_state_to_memento());
    originator.set_state("State #4".to_owned());
    let state4 = originator.save_state_to_memento();
    caretaker.add(state4);

    println!("Current State: {}", originator.get_state());
    originator.get_state_from_memento(caretaker.get(0).unwrap());
    println!("First Saved State: {}", originator.get_state());
    originator.get_state_from_memento(caretaker.get(1).unwrap());
    println!("Second Saved State: {}", originator.get_state());
}
