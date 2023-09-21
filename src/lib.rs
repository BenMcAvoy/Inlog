mod colours;

use log::{Level, LevelFilter, Metadata, Record};
use crate::colours::colour_format;

struct Analog;

impl log::Log for Analog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        log::set_max_level(LevelFilter::Debug);
        metadata.level() <= Level::Debug
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

        println!(
            "{} {}",
            colour_format(colour, &format!("[ {:5} ]", level)),
            record.args()
        );
    }

    fn flush(&self) {}
}

static LOGGER: Analog = Analog;

pub fn init(filter: LevelFilter) {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(filter))
        .expect("The logger couldn't be set. Quitting.");
}
