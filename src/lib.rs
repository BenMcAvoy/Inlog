use log::{SetLoggerError, LevelFilter};
use log::{Record, Level, Metadata};

use crate::colours::colour_format;

mod colours;

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        log::set_max_level(LevelFilter::Trace);
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        // Guard for if logger is enabled
        if !self.enabled(record.metadata()) {
            return;
        }

        let level = record.level();

        let colour = match level {
            Level::Trace => colours::LIGHT_BLUE,
            Level::Debug => colours::BLUE,
            Level::Info => colours::GREEN,
            Level::Warn => colours::YELLOW,
            Level::Error => colours::RED,
        };

        println!("{} {}", colour_format(colour, &format!("[ {:5} ]", level)), record.args());
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init(filter: LevelFilter) {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(filter))
        .expect("The logger couldn't be set. Quitting.");
}
