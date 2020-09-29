#[derive(PartialOrd, PartialEq)]
pub enum LogLevel {
    INFO = 1,
    DEBUG = 2,
    ERROR = 3,
}

pub trait AbstractLogger {
    fn write(&self, _: &str);
}

pub struct Logger<'a> {
    level: LogLevel,
    logger: Box<dyn AbstractLogger>,
    next_logger: Option<&'a Logger<'a>>,
}

impl<'a> Logger<'a> {
    pub fn new(level: LogLevel, logger: Box<dyn AbstractLogger>) -> Logger<'a> {
        Logger {
            level,
            logger,
            next_logger: None,
        }
    }

    pub fn set_next_logger(&mut self, next_logger: &'a Logger<'a>) {
        self.next_logger = Some(next_logger);
    }
    //
    pub fn log_message(&self, level: LogLevel, message: &str) {
        if self.level <= level {
            self.logger.write(message);
        }
        if self.next_logger.is_some() {
            self.next_logger.unwrap().log_message(level, message)
        }
    }
}

pub struct ConsoleLogger {}

impl AbstractLogger for ConsoleLogger {
    fn write(&self, message: &str) {
        println!("Standard Console:logger: {}", message)
    }
}

pub struct ErrorLogger {}

impl AbstractLogger for ErrorLogger {
    fn write(&self, message: &str) {
        println!("Error Console:logger: {}", message)
    }
}

pub struct FileLogger {}

impl AbstractLogger for FileLogger {
    fn write(&self, message: &str) {
        println!("File Console:logger: {}", message)
    }
}
