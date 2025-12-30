use std::path::PathBuf;
use std::sync::Once;
use std::{panic, thread};
use tracing::{Level, error};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{
    EnvFilter, Layer, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt,
};

static INIT: Once = Once::new();

/// Configuration for the application telemetry (logging).
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    /// Name of the application.
    /// Used for log filenames: `app_name.log.YYYY-MM-DD`.
    pub app_name: String,

    /// Directory to store log files.
    /// Defaults to explicit `None` (console only) unless set.
    pub log_dir: Option<PathBuf>,

    /// The lowest log level to record for the application itself.
    /// Defaults to `Level::INFO`.
    pub level: Level,

    /// Whether to print logs to standard output (console).
    pub use_console: bool,

    /// List of crates/modules to silence.
    /// These will be restricted to ERROR level.
    pub silent_crates: Vec<String>,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            app_name: "app".to_string(),
            log_dir: None,
            level: Level::INFO,
            use_console: true,
            silent_crates: Vec::new(),
        }
    }
}

impl TelemetryConfig {
    /// Start a new configuration with a specific application name.
    pub fn new(app_name: impl Into<String>) -> Self {
        Self {
            app_name: app_name.into(),
            ..Default::default()
        }
    }

    /// Enable file logging to the specified directory.
    pub fn with_file_logging(mut self, dir: impl Into<PathBuf>) -> Self {
        self.log_dir = Some(dir.into());
        self
    }

    /// Set the verbosity level (e.g., Level::DEBUG or Level::TRACE).
    pub fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Disable console output (useful for background services).
    pub fn hide_console(mut self) -> Self {
        self.use_console = false;
        self
    }

    /// Add a list of crates to silence (restrict to WARN level).
    pub fn silence_crates(mut self, crates: &[&str]) -> Self {
        self.silent_crates = crates.iter().map(|&s| s.to_string()).collect();
        self
    }
}

pub struct TelemetryHandle {
    /// The guard for the non-blocking file writer.
    /// **MUST** be held by `main` to ensure logs flush on shutdown.
    pub guard: Option<WorkerGuard>,
    /// The full path to the log file (directory + filename).
    pub log_file: Option<PathBuf>,
}

/// Initialize the telemetry system.
///
/// # Returns
/// A `TelemetryHandle` containing the worker guard and log filepath.
pub fn init(config: TelemetryConfig) -> Result<TelemetryHandle, Box<dyn std::error::Error>> {
    let mut handle = Err("Logger is already initialized".into());

    INIT.call_once(|| {
        handle = init_internal(config);
    });

    handle
}

fn init_internal(config: TelemetryConfig) -> Result<TelemetryHandle, Box<dyn std::error::Error>> {
    let mut filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new(format!("{}={}", config.app_name, config.level))
            .add_directive(config.level.into())
    });

    for crate_name in &config.silent_crates {
        filter = filter.add_directive(format!("{}={}", crate_name, "error").parse()?);
    }

    let mut layers = Vec::new();
    let mut file_guard = None;
    let mut log_file = None;

    if let Some(log_dir) = config.log_dir {
        std::fs::create_dir_all(&log_dir)?;

        let filename = format!("{}.log", config.app_name);

        // Construct the full path so the handle returns the correct location
        log_file = Some(log_dir.join(&filename));

        let file_appender = RollingFileAppender::new(Rotation::DAILY, log_dir, filename);

        // PERFORMANCE: Offload writing to a background thread
        let (non_blocking_writer, guard) = tracing_appender::non_blocking(file_appender);
        file_guard = Some(guard);

        let file_layer = fmt::Layer::new()
            .with_writer(non_blocking_writer)
            .with_ansi(false) // No colors in files
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(true);

        layers.push(file_layer.with_filter(filter.clone()).boxed());
    }

    if config.use_console {
        let console_layer = fmt::Layer::new()
            .with_ansi(true) // Pretty colors in console
            .with_thread_ids(true)
            .compact();

        layers.push(console_layer.with_filter(filter).boxed());
    }

    // Attempt to register the subscriber.
    // If it fails (e.g., in tests), we log a warning but return Ok to prevent panics.
    if let Err(e) = registry().with(layers).try_init() {
        eprintln!("WARN: Telemetry already initialized: {}", e);
    }

    set_panic_hook();

    Ok(TelemetryHandle {
        guard: file_guard,
        log_file,
    })
}

fn set_panic_hook() {
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let payload = panic_info.payload();

        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            *s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.as_str()
        } else {
            "panic occurred"
        };

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_else(|| "unknown location".to_string());

        let thread = thread::current();
        let thread_name = thread.name().unwrap_or("<unnamed>");

        error!(
            thread = thread_name,
            panic.location = location,
            panic.message = message,
            "CRITICAL: Application Panicked"
        );

        prev_hook(panic_info);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = TelemetryConfig::new("test_app_name")
            .with_level(Level::DEBUG)
            .hide_console()
            .silence_crates(&["crate_a", "crate_b"]);

        assert_eq!(config.app_name, "test_app_name");
        assert_eq!(config.level, Level::DEBUG);
        assert_eq!(config.use_console, false);
        assert_eq!(config.silent_crates.len(), 2);
    }

    #[test]
    fn test_telemetry_lifecycle() {
        let log_dir = std::env::temp_dir().join("app_tests");

        let config = TelemetryConfig::new("test_app")
            .with_level(Level::DEBUG)
            .with_file_logging(log_dir.clone());

        let handle_res = init(config);

        // This will now pass even if other tests initialized tracing
        assert!(handle_res.is_ok());

        let handle = handle_res.unwrap();

        // If this is the first time init ran, verify the path.
        if let Some(path) = handle.log_file {
            assert_eq!(path, log_dir.join("test_app.log"));
        }
    }
}
