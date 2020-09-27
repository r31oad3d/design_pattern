#![allow(dead_code)]
use std::cell::RefCell;
use std::fs::File;

pub trait Image {
    fn display(&self);
}

pub struct RealImage {
    file_name: String,
    file: std::io::Result<File>,
}

impl RealImage {
    pub fn new(file_name: String) -> RealImage {
        println!("RealImage newed");
        RealImage {
            file_name: file_name.clone(),
            file: File::open(file_name),
        }
    }
}

pub struct ProxyImage {
    file_name: String,
    real_image: RefCell<Option<RealImage>>,
}

impl ProxyImage {
    pub fn new(file_name: String) -> ProxyImage {
        ProxyImage {
            file_name,
            real_image: RefCell::new(None),
        }
    }
}

impl Image for RealImage {
    fn display(&self) {
        println!("Displaying: {:?}", self.file_name);
    }
}

impl Image for ProxyImage {
    fn display(&self) {
        let mut temp = self.real_image.borrow_mut();
        if temp.as_ref().is_none() {
            *temp = Some(RealImage::new(self.file_name.clone()));
        }
        temp.as_ref().unwrap().display()
    }
}
