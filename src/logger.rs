use log::{self, Log, LevelFilter, Level, Record, SetLoggerError, Metadata};
use ::std::fmt::Write;

pub struct Logger {
    filter: LevelFilter,
}

impl Logger {
    /// Returns the maximum `LevelFilter` that this logger instance is
    /// configured to output.
    pub fn filter(&self) -> LevelFilter {
        self.filter
    }

    pub fn try_init_with_level(level: LevelFilter) -> Result<(), SetLoggerError> {
        let logger = Self {
            filter: level,
        };

        log::set_max_level(logger.filter());
        log::set_boxed_logger(Box::new(logger))
    }

    pub fn init_with_level(level: LevelFilter) {
        Self::try_init_with_level(level).unwrap();
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.filter >= metadata.level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let mut message = String::new();
        write!(&mut message, "{}", record.args()).unwrap();

        match record.level() {
            Level::Error => {
                js!{ console.error(@{message}); };
            },
            Level::Warn => {
                js!{ console.warn(@{message}); };
            },
            Level::Info => {
                js!{ console.info(@{message}); };
            },
            Level::Debug => {
                js!{ console.debug(@{message}); };
            },
            Level::Trace => {
                // There is a console.trace(), but that only outputs a stacktrace and not a
                // supplied message, so we'll be using console.debug instead.
                js!{ console.debug(@{message}); };
            },
        }
    }

    fn flush(&self) {}
}
