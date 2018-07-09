//! Simple console logger that is configurable at runtime for command line apps.
//!
//! ## Example
//!
//! ```
//! extern crate clogger;
//! #[macro_use]
//! extern crate log;
//!
//! fn main() {
//!     clogger::init();
//!
//!     debug!("this is a debug message and will be hidden");
//!     error!("this is printed by default");
//!
//!     clogger::set_verbosity(2);
//!
//!     debug!("verbosity increased, this will now be displayed");
//! }
//! ```
extern crate ansi_term;
extern crate log;

use ansi_term::Color;
use log::*;
use std::sync::atomic::*;

static INSTANCE: Logger = Logger {
    quiet: AtomicBool::new(false),
    verbosity: AtomicUsize::new(0),
};

struct Logger {
    quiet: AtomicBool,
    verbosity: AtomicUsize,
}

impl Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let (name, color) = match record.metadata().level() {
            Level::Error => ("error", Color::Red),
            Level::Warn => ("warn", Color::Purple),
            Level::Info => ("info", Color::Yellow),
            Level::Debug => ("debug", Color::Cyan),
            Level::Trace => ("trace", Color::Blue),
        };

        eprintln!("{}: {}", color.paint(name), record.args());
    }

    fn flush(&self) {}
}

/// Initialize the global logger.
///
/// This function should be called at the beginning of your application so that all log messages are handled.
///
/// This function may only be called once. Panics if initialization fails.
pub fn init() {
    try_init().expect("logger failed to initialize");
}

/// Attempts to initialize the global logger.
pub fn try_init() -> Result<(), SetLoggerError> {
    update_max_level();
    set_logger(&INSTANCE)
}

/// Check if quiet mode is enabled.
pub fn quiet() -> bool {
    INSTANCE.quiet.load(Ordering::SeqCst)
}

/// Turn quiet mode on or off.
///
/// When quiet mode is enabled, all logging output is discarded.
///
/// This function may be called at any time.
pub fn set_quiet(enabled: bool) {
    INSTANCE.quiet.store(enabled, Ordering::SeqCst);
    update_max_level();
}

/// Get the current logger verbosity level.
pub fn verbosity() -> usize {
    INSTANCE.verbosity.load(Ordering::SeqCst)
}

/// Set the current logger verbosity level.
///
/// The verbosity level controls the maximum log level that is displayed. A verbosity of `0` sets the max log level to
/// `Warn`, a level of `1` sets the max log level to `Info`, and so on.
///
/// This function may be called at any time.
pub fn set_verbosity(verbosity: usize) {
    INSTANCE.verbosity.store(verbosity, Ordering::SeqCst);
    update_max_level();
}

fn update_max_level() {
    set_max_level(if quiet() {
        LevelFilter::Off
    } else {
        match verbosity() {
            0 => LevelFilter::Warn,
            1 => LevelFilter::Info,
            2 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        }
    });
}
