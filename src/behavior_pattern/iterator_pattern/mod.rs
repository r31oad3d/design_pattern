pub trait Iterator<'a> {
    fn has_next() -> bool;
    fn next(&mut self) -> &str;
}

