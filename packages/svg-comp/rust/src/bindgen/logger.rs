#[cfg(feature = "console_log")]
mod console {
    use log::{Level, Log, Metadata, Record};
    use once_cell::sync::OnceCell;

    use super::js_bindings;

    // Using OnceCell for safe initialization
    static DEFAULT_LOGGER: OnceCell<ConsoleLogger> = OnceCell::new();

    pub fn default_logger() -> &'static ConsoleLogger {
        DEFAULT_LOGGER.get_or_init(|| ConsoleLogger::new(Formatter::Default, Level::Trace))
    }

    pub enum Formatter {
        Default,
        Custom(Box<dyn Fn(&Record) -> String + Send + Sync>),
    }

    impl Formatter {
        fn format(&self, record: &Record) -> String {
            match self {
                Self::Default => format!("{}: {}", record.level(), record.args()),
                Self::Custom(func) => func(record),
            }
        }
    }

    pub struct ConsoleLogger {
        formatter: Formatter,
        log_level: Level,
    }

    impl ConsoleLogger {
        pub fn new(formatter: Formatter, log_level: Level) -> Self {
            ConsoleLogger {
                formatter,
                log_level,
            }
        }
    }

    impl Log for ConsoleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= self.log_level
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let msg = self.formatter.format(record);
                match record.level() {
                    Level::Error => js_bindings::error(&msg),
                    Level::Warn => js_bindings::warn(&msg),
                    _ => js_bindings::log(&msg),
                }
            }
        }

        fn flush(&self) {}
    }
}
