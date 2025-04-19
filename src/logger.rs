use env_logger::Builder;
use log::{Level, LevelFilter, Metadata, Record};
use std::sync::Mutex;
use egui::{Color32, RichText};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LAST_LOG_LINE: Mutex<RichText> = Mutex::new(RichText::new(""));
}

struct CaptureLogger {
    env_logger: env_logger::Logger,
}

impl log::Log for CaptureLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.env_logger.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let mut last_line = LAST_LOG_LINE.lock().unwrap();
            *last_line = match record.level() {
                Level::Error => RichText::new(format!("{}", record.args())).color(Color32::RED),
                Level::Warn => RichText::new(format!("{}", record.args())).color(Color32::LIGHT_YELLOW),
                Level::Info => RichText::new(format!("{}", record.args())),
                Level::Debug => RichText::new(format!("{}", record.args())),
                Level::Trace => RichText::new(format!("{}", record.args())),
            };

            self.env_logger.log(record);
        }
    }

    fn flush(&self) {
        self.env_logger.flush();
    }
}

pub fn init_logger(filter: LevelFilter) {
    let env_logger = Builder::new().filter_level(filter).build();
    let logger = CaptureLogger { env_logger };
    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(filter);
}