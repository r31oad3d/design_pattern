pub trait Iterator<'a> {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> Option<&str>;
}


pub struct NameIterator<'a> {
    names: &'a [&'a str],
    index: usize,
}

impl<'a> NameIterator<'a> {
    pub fn new(names: &'a [&'a str]) -> Self {
        NameIterator {names, index:0}
    }
}

impl<'a> Iterator<'a> for NameIterator<'a> {
    fn has_next(&self) -> bool {
        self.index < self.names.len()
    }

    fn next(&mut self) -> Option<&str> {
        if self.has_next() {
            let temp = self.index;
            self.index += 1;
            Some(self.names[temp])
        } else {
            None
        }

    }
}
