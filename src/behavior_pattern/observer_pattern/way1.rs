use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result};
use std::rc::{Rc, Weak};


pub trait Observer {
    fn update(&self);
}

pub struct Subject {
    observers: RefCell<Vec<Rc<dyn Observer>>>,
    state: RefCell<i32>,
}

impl Subject {
    pub fn new() -> Subject {
        Subject {
            observers: RefCell::new(Vec::new()),
            state: RefCell::new(0),
        }
    }

    pub fn get_state(&self) -> i32 {
        *self.state.borrow()
    }

    pub fn set_state(&self, state: i32) {
        *self.state.borrow_mut() = state;
        self.notify_all_observer();
    }

    pub fn attach(&self, observer: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(observer);
    }

    pub fn notify_all_observer(&self) {
        println!("{:?}", self.observers);
        self.observers.borrow()
            .iter()
            .for_each(|observer| observer.update());
    }
}

pub struct BinaryObserver {
    subject: Weak<Subject>,
}
pub struct OctalObserver {
    subject: Weak<Subject>,
}
pub struct HexaObserver {
    subject: Weak<Subject>,
}

impl BinaryObserver {
    pub fn new(subject: Rc<Subject>) -> Rc<BinaryObserver> {
        Rc::new(BinaryObserver {
            subject: Rc::downgrade(&subject),
        })
    }
}

impl OctalObserver {
    pub fn new(subject: Rc<Subject>) -> Rc<OctalObserver> {
        Rc::new(OctalObserver {
            subject: Rc::downgrade(&subject),
        })
    }
}

impl HexaObserver {
    pub fn new(subject: Rc<Subject>) -> Rc<HexaObserver> {
        Rc::new(HexaObserver {
            subject: Rc::downgrade(&subject),
        })
    }
}

impl Observer for BinaryObserver {
    fn update(&self) {
        println!("Binary String: {:b}", *self.subject.upgrade().unwrap().state.borrow())
    }
}

impl Observer for OctalObserver {
    fn update(&self) {
        println!("Octal String: {:o}", *self.subject.upgrade().unwrap().state.borrow())
    }
}

impl Observer for HexaObserver {
    fn update(&self) {
        println!("Hex String: {:X}", *self.subject.upgrade().unwrap().state.borrow())
    }
}

impl Debug for dyn Observer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "dyn")
    }
}
