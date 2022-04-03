use std::any::Any;

pub trait ComputerPart {
    fn accept(&self, visitor: &ComputerPartDisplayVisitor);
    fn as_any(&self) -> &dyn Any;
}

pub struct Keyboard;
pub struct Monitor;
pub struct Mouse;
pub struct Computer {
    parts: Vec<Box<dyn ComputerPart>>,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            parts: vec![Box::new(Mouse), Box::new(Keyboard), Box::new(Monitor)],
        }
    }
}
impl ComputerPart for Keyboard {
    fn accept(&self, visitor: &ComputerPartDisplayVisitor) {
        visitor.visit(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ComputerPart for Monitor {
    fn accept(&self, visitor: &ComputerPartDisplayVisitor) {
        visitor.visit(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ComputerPart for Mouse {
    fn accept(&self, visitor: &ComputerPartDisplayVisitor) {
        visitor.visit(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ComputerPart for Computer {
    fn accept(&self, visitor: &ComputerPartDisplayVisitor) {
        self.parts.iter().for_each(|part| part.accept(visitor));
        visitor.visit(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait ComputerVisitor {
    fn visit(&self, computer_part: &dyn Any);
}
pub struct ComputerPartDisplayVisitor;

impl ComputerVisitor for ComputerPartDisplayVisitor {
    fn visit(&self, computer_part: &dyn Any) {
        // ugly
        if computer_part.downcast_ref::<Mouse>().is_some() {
            println!("Displaying Mouse.")
        }
        if computer_part.downcast_ref::<Keyboard>().is_some() {
            println!("Displaying Keyboard.")
        }
        if computer_part.downcast_ref::<Monitor>().is_some() {
            println!("Displaying Monitor.")
        }
        if computer_part.downcast_ref::<Computer>().is_some() {
            println!("Displaying Computer.")
        }
    }
}
