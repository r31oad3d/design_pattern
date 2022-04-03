#![allow(dead_code)]
use design_pattern::structural_pattern::proxy_pattern::way1::{
    Image, ProxyImage,
};

fn main() {
    let image = ProxyImage::new("test.png".to_owned());
    image.display();
    println!();
    image.display();
}
