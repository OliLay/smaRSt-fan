use log::LevelFilter;
use simple_logger::SimpleLogger;

pub fn initialize_logging(level: LevelFilter) {
    SimpleLogger::new().with_level(level).init().unwrap();
}
