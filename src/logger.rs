use log::{LevelFilter, Log, Metadata, Record};
pub struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init_logging(max_level: LevelFilter) {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(max_level))
        .expect("Unable to initialize logging");
}
