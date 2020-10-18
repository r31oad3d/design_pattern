use design_pattern::behavior_pattern::state_pattern::way1::{Context, State};
use std::rc::Rc;

fn main() {
    let start_state = Rc::new(State::StartState);
    let mut context = Context::new(start_state.clone());

    State::do_action(start_state, &mut context);
    println!("{}", context.get_state());

    let stop_state = Rc::new(State::StopState);
    State::do_action(stop_state, &mut context);
    println!("{}", context.get_state());
}
