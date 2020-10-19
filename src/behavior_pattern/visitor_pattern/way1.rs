pub trait ComputerPart {
    fn accept(&self, visitor: &ComputerPartDisplayVisitor);
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
}

impl ComputerPart for Monitor {
    fn accept(&self, visitor: &ComputerPartDisplayVisitor) {
        visitor.visit(self);
    }
}

impl ComputerPart for Mouse {
    fn accept(&self, visitor: &ComputerPartDisplayVisitor) {
        visitor.visit(self);
    }
}

impl ComputerPart for Computer {
    fn accept(&self, visitor: &ComputerPartDisplayVisitor) {
        self.parts.iter().for_each(|part| part.accept(visitor));
        visitor.visit(self);
    }
}

pub trait ComputerVisitor<T>
where
    T: ComputerPart,
{
    fn visit(&self, computer_part: &T);
}
pub struct ComputerPartDisplayVisitor;

impl ComputerVisitor<Keyboard> for ComputerPartDisplayVisitor {
    fn visit(&self, _computer_part: &Keyboard) {
        println!("Displaying Keyboard.")
    }
}

impl ComputerVisitor<Monitor> for ComputerPartDisplayVisitor {
    fn visit(&self, _computer_part: &Monitor) {
        println!("Displaying Monitor.")
    }
}

impl ComputerVisitor<Mouse> for ComputerPartDisplayVisitor {
    fn visit(&self, _computer_part: &Mouse) {
        println!("Displaying Mouse.")
    }
}

impl ComputerVisitor<Computer> for ComputerPartDisplayVisitor {
    fn visit(&self, _computer_part: &Computer) {
        println!("Displaying Computer.")
    }
}
