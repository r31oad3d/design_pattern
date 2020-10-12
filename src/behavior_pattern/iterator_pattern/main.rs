use design_pattern::behavior_pattern::iterator_pattern::{
    Iterator, NameIterator,
};

fn main() {
    let mut name_iterator =
        NameIterator::new(&["Robert", "John", "Julie", "Lora"]);
    while let Some(name) = name_iterator.next() {
        println!("name: {}", name);
    }
}
