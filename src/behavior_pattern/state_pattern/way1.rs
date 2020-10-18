use std::rc::Rc;
use std::fmt::{Display, Formatter, Result};
use std::ops::Deref;


pub enum State {
    StartState,
    StopState,
}

impl  State  {
    pub fn do_action(state: Rc<State>, context: &mut Context) {
        match state.deref() {
            State::StartState => {
                println!("Player is in start state");
            },
            State::StopState => {
                println!("Player is in stop state");
            },
        }
        context.set_state(state);
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            State::StartState => {
                write!(f, "Start State")
            },
            State::StopState => {
                write!(f, "Stop State")
            },
        }
    }
}

pub struct Context{
    state: Rc<State>,
}

impl Context{
    pub fn new(state: Rc<State>) -> Context {
        Context {
            state
        }
    }

    pub fn set_state(&mut self, state: Rc<State>) {
        self.state = state;
    }

    pub fn get_state(&self) -> Rc<State> {
        Rc::clone(&self.state)
    }

}



