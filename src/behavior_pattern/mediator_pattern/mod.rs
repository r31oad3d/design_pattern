use chrono::prelude::*;

pub struct ChatRoom;

impl ChatRoom {
    pub fn show_message(user: &User, message: &str) {
        println!(
            "{} - [{}] : {}",
            Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            user.get_name(),
            message
        )
    }
}

pub struct User<'a> {
    name: &'a str,
}

impl<'a> User<'a> {
    pub fn new(name: &'a str) -> Self {
        User { name }
    }

    pub fn get_name(&self) -> &str {
        self.name
    }

    pub fn send_message(&self, message: String) {
        ChatRoom::show_message(self, message.as_str())
    }
}
