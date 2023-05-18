use log::{self, Level, LevelFilter, Log, Metadata, Record};

use crate::println;


struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let color = match record.level() {
            Level::Error => 31, // Red
            Level::Warn => 93,  // BrightYellow
            Level::Info => 34,  // Blue
            Level::Debug => 32, // Green
            Level::Trace => 90, // BrightBlack
        };
        println!(
            "\u{1B}[{}m[{:>5}] {}\u{1B}[0m",
            color,
            record.level(),
            record.args(),
        );
    }
    fn flush(&self) {}
}

#[allow(dead_code)]
pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}


#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!("\x1B[90m{}\x1B[0m", format_args!(concat!($fmt, "\n") $(, $($arg)+)?)));
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!("\x1B[32m{}\x1B[0m", format_args!(concat!($fmt, "\n") $(, $($arg)+)?)));
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!("\x1B[34m{}\x1B[0m", format_args!(concat!($fmt, "\n") $(, $($arg)+)?)));
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!("\x1B[93m{}\x1B[0m", format_args!(concat!($fmt, "\n") $(, $($arg)+)?)));
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!("\x1B[31m{}\x1B[0m", format_args!(concat!($fmt, "\n") $(, $($arg)+)?)));
    }
}