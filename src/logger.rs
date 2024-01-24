// logger.rs
use lazy_static::lazy_static;
use log::{error, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use uuid::Uuid;

lazy_static! {
    pub static ref LOGGER: MyLogger = MyLogger::new();
}

pub struct MyLogger {
    request_id: String,
}

impl MyLogger {
    pub fn new() -> MyLogger {
        MyLogger {
            request_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn init_logger(&self) {
        let console = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{h({l})} - {d(%Y-%m-%d %H:%M:%S)} - {m}{n}",
            )))
            .build();

        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "{h({l})} - {d(%Y-%m-%d %H:%M:%S)} - {m}{n}",
            )))
            .build("monitoring.log")
            .unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("console", Box::new(console)))
            .appender(Appender::builder().build("file", Box::new(file)))
            .build(
                Root::builder()
                    .appender("console")
                    .appender("file")
                    .build(LevelFilter::Info),
            )
            .unwrap();

        log4rs::init_config(config).unwrap();
    }

    pub fn log_error(&self, message: &str) {
        error!("{}", self.format_log(message));
    }

    pub fn log_info(&self, message: &str) {
        info!("{}", self.format_log(message));
    }

    fn format_log(&self, message: &str) -> String {
        format!("{}, {}", self.request_id, message)
    }
}
