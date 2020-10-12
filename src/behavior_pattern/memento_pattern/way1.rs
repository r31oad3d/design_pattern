pub struct Memento {
    state: String,
}

impl Memento {
    pub fn new(state: String) -> Memento {
        Memento { state }
    }

    pub fn get_state(&self) -> String {
        self.state.clone()
    }
}

pub struct Originator {
    state: String,
}

impl Originator {
    pub fn new(state: String) -> Originator {
        Originator { state }
    }

    pub fn set_state(&mut self, state: String) {
        self.state = state
    }

    pub fn get_state(&self) -> &String {
        &self.state
    }

    pub fn save_state_to_memento(&self) -> Memento {
        Memento {
            state: self.state.clone(),
        }
    }

    pub fn get_state_from_memento(&mut self, memento: &Memento) {
        self.state = memento.get_state()
    }
}

pub struct CareTaker {
    memento_list: Vec<Memento>,
}

impl CareTaker {
    pub fn new() -> CareTaker {
        CareTaker {
            memento_list: Vec::<Memento>::new(),
        }
    }

    pub fn add(&mut self, state: Memento) {
        self.memento_list.push(state)
    }

    pub fn get(&self, index: usize) -> Option<&Memento> {
        self.memento_list.get(index)
    }
}
