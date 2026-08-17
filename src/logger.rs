use simplelog::*;
use std::fs::{create_dir_all, File};
use std::path::PathBuf;

pub fn init() {
    let log_dir = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".apdl")
        .join("logs");

    let _ = create_dir_all(&log_dir);
    let log_file_path = log_dir.join("app.log");

    let config = ConfigBuilder::new()
        .set_time_format_rfc3339()
        .set_time_offset_to_local()
        .unwrap_or_else(|b| b)
        .build();

    let mut loggers: Vec<Box<dyn SharedLogger>> = Vec::new();

    let term = TermLogger::new(
        LevelFilter::Info,
        config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    loggers.push(term);

    if let Ok(file) = File::create(&log_file_path) {
        loggers.push(WriteLogger::new(LevelFilter::Debug, config, file));
    }

    let _ = CombinedLogger::init(loggers);
    log::info!("APDL initialized. Logging to {:?}", log_file_path);
}
