use design_pattern::behavior_pattern::chain_of_responsibility_pattern::{
    ConsoleLogger, ErrorLogger, FileLogger, LogLevel, Logger,
};

fn main() {
    let mut error_logger =
        Logger::new(LogLevel::ERROR, Box::new(ErrorLogger {}));
    let mut file_logger = Logger::new(LogLevel::DEBUG, Box::new(FileLogger {}));
    let mut console_logger =
        Logger::new(LogLevel::INFO, Box::new(ConsoleLogger {}));
    file_logger.set_next_logger(&console_logger);
    error_logger.set_next_logger(&file_logger);

    error_logger.log_message(LogLevel::INFO, "This is an information.");
    error_logger
        .log_message(LogLevel::DEBUG, "This is a debug level information.");
    error_logger.log_message(LogLevel::ERROR, "This is an error information.");
}
